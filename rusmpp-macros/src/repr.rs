use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::container_attributes::{
    DecodeAttributes, DecodeImplementation, FromIntoAttributes, TestAttributes,
};

pub struct Repr {
    ident: Ident,
}

impl Repr {
    pub fn new(repr_type: ReprType) -> Self {
        let ident = repr_type.to_ident();

        Self { ident }
    }
}

pub enum ReprType {
    U8,
    U16,
    U32,
}

impl ReprType {
    fn to_ident(&self) -> Ident {
        match self {
            Self::U8 => Ident::new("u8", proc_macro2::Span::call_site()),
            Self::U16 => Ident::new("u16", proc_macro2::Span::call_site()),
            Self::U32 => Ident::new("u32", proc_macro2::Span::call_site()),
        }
    }
}

impl Repr {
    fn quote_length_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl crate::encode::Length for #name {
                fn length(&self) -> usize {
                    #repr_ident::from(*self).length()
                }
            }
        }
    }

    fn quote_encode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl crate::encode::Encode for #name {
                fn encode(&self, dst: &mut [u8]) -> usize {
                    #repr_ident::from(*self).encode(dst)
                }
            }

            #[cfg(feature = "alloc")]
            impl crate::encode::owned::Encode for #name {
                fn encode(&self, dst: &mut ::bytes::BytesMut){
                    #repr_ident::from(*self).encode(dst)
                }
            }
        }
    }

    fn quote_owned_decode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            #[cfg(feature = "alloc")]
            impl crate::decode::owned::Decode for #name {
                fn decode(src: &mut ::bytes::BytesMut) -> Result<(Self, usize), crate::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }

    fn quote_borrowed_decode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl<'a> crate::decode::borrowed::Decode<'a> for #name {
                fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }

    fn quote_decode_impl(&self, name: &Ident, decode_attrs: &DecodeAttributes) -> TokenStream {
        match decode_attrs {
            DecodeAttributes::Skip => quote! {},
            DecodeAttributes::Implement(impl_type) => match impl_type {
                DecodeImplementation::Owned => self.quote_owned_decode_impl(name),
                DecodeImplementation::Borrowed => self.quote_borrowed_decode_impl(name),
                DecodeImplementation::All => {
                    let owned = self.quote_owned_decode_impl(name);
                    let borrowed = self.quote_borrowed_decode_impl(name);

                    quote! {
                        #owned
                        #borrowed
                    }
                }
            },
        }
    }

    fn quote_test_impl(&self, name: &Ident, test_attrs: &TestAttributes) -> TokenStream {
        match test_attrs {
            TestAttributes::Skip => quote! {},
            TestAttributes::Implement => {
                quote! {
                    #[cfg(test)]
                    impl crate::tests::TestInstance for #name {
                        fn instances() -> alloc::vec::Vec<Self> {
                            alloc::vec![Self::default(),]
                        }
                    }
                }
            }
        }
    }

    fn quote_from_into_impl(&self, input: &DeriveInput) -> TokenStream {
        let name = &input.ident;
        let repr_ident = &self.ident; // u8, u16, u32

        let variants = if let syn::Data::Enum(ref data_enum) = input.data {
            data_enum
                .variants
                .iter()
                .map(|v| {
                    let ident = &v.ident;
                    v.discriminant
                        .as_ref()
                        .map(|(_, expr)| (ident.clone(), expr.clone()))
                })
                .collect::<Vec<_>>()
        } else {
            return quote! {};
        };

        // From<repr> for Enum
        let from_matches = variants.iter().filter_map(|opt| {
            opt.as_ref().map(|(ident, expr)| {
                quote! { #expr => #name::#ident, }
            })
        });

        // From<Enum> for repr
        let into_matches = variants.iter().filter_map(|opt| {
            opt.as_ref().map(|(ident, expr)| {
                quote! { #name::#ident => #expr, }
            })
        });

        quote! {
            impl From<#repr_ident> for #name {
                fn from(value: #repr_ident) -> Self {
                    match value {
                        #(#from_matches)*
                        other => #name::Other(other),
                    }
                }
            }

            impl From<#name> for #repr_ident {
                fn from(value: #name) -> Self {
                    match value {
                        #(#into_matches)*
                        #name::Other(other) => other,
                    }
                }
            }
        }
    }

    pub fn quote_rusmpp(
        &self,
        input: &DeriveInput,
        from_into_attrs: FromIntoAttributes,
        decode_attrs: &DecodeAttributes,
        test_attrs: &TestAttributes,
    ) -> TokenStream {
        let _ = from_into_attrs;
        let name = &input.ident;

        let length_impl = self.quote_length_impl(name);
        let encode_impl = self.quote_encode_impl(name);
        let decode_impl = self.quote_decode_impl(name, decode_attrs);
        let test_impl = self.quote_test_impl(name, test_attrs);

        let from_into_impl = if from_into_attrs.is_implement() {
            self.quote_from_into_impl(input)
        } else {
            quote! {}
        };

        quote! {
            #length_impl
            #encode_impl
            #decode_impl
            #test_impl
            #from_into_impl
        }
    }
}
