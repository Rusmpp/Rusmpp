use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{DeriveInput, Field, FieldsNamed, Ident, Lit};

use crate::{
    container_attributes::{
        DecodeAttributes, DecodeImplementation, FromIntoAttributes, TestAttributes,
    },
    parts,
    repr::{Repr, ReprType},
};

pub fn derive_rusmpp_for_struct(
    input: &DeriveInput,
    fields_named: &FieldsNamed,
) -> syn::Result<TokenStream> {
    let struct_attrs = StructAttributes::extract(input)?;

    let parts = parts::quote_parts(input, fields_named);

    if let Some(repr) = struct_attrs.repr {
        let repr_expanded = repr.quote_rusmpp(
            input,
            FromIntoAttributes::Skip,
            &struct_attrs.decode_attrs,
            &struct_attrs.test_attrs,
        );

        let expanded = quote! {
            #parts
            #repr_expanded
        };

        return Ok(expanded);
    }

    let length = quote_length(input, fields_named);
    let encode = quote_encode(input, fields_named);
    let decode = quote_decode(input, fields_named, &struct_attrs.decode_attrs)?;
    let test = quote_test(input, &struct_attrs.test_attrs);

    let expanded = quote! {
        #parts
        #length
        #encode
        #decode
        #test
    };

    Ok(expanded)
}

fn quote_length(input: &DeriveInput, fields_named: &FieldsNamed) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let field_idents = fields_named
        .named
        .iter()
        .map(|f| f.ident.as_ref().expect("Named fields must have idents"));

    quote! {
        impl #impl_generics crate::encode::Length for #name #ty_generics #where_clause {
            fn length(&self) -> usize {
                let mut length = 0;
                #(
                    length += crate::encode::Length::length(&self.#field_idents);
                )*
                length
            }
        }
    }
}

fn quote_encode(input: &DeriveInput, fields_named: &FieldsNamed) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let field_idents = fields_named
        .named
        .iter()
        .map(|f| f.ident.as_ref().expect("Named fields must have idents"))
        .collect::<Vec<_>>();

    quote! {
        impl #impl_generics crate::encode::Encode for #name #ty_generics #where_clause {
            fn encode(&self, dst: &mut [u8]) -> usize {
                let size = 0;
                #(
                    let size = crate::encode::EncodeExt::encode_move(&self.#field_idents, dst, size);
                )*
                size
            }
        }

        #[cfg(feature = "alloc")]
        impl #impl_generics crate::encode::owned::Encode for #name #ty_generics #where_clause {
            fn encode(&self, dst: &mut ::bytes::BytesMut){
                #(
                    crate::encode::owned::Encode::encode(&self.#field_idents, dst);
                )*
            }
        }
    }
}

fn quote_decode(
    input: &DeriveInput,
    fields_named: &FieldsNamed,
    decode_attrs: &DecodeAttributes,
) -> syn::Result<TokenStream> {
    match decode_attrs {
        DecodeAttributes::Skip => Ok(quote! {}),
        DecodeAttributes::Implement(impl_type) => {
            let fields: ValidFields = fields_named
                .named
                .iter()
                .map(
                    |field| match FieldAttributes::extract(field).and_then(|a| a.validated()) {
                        Ok(attrs) => Ok(ValidField { field, attrs }),
                        Err(err) => Err(syn::Error::new_spanned(field, err)),
                    },
                )
                .collect::<Result<Vec<_>, _>>()?
                .into();

            let decode_type = fields.decode_type();

            match impl_type {
                DecodeImplementation::Owned => match decode_type {
                    DecodeType::Decode => Ok(quote_owned_decode(input, &fields)),
                    DecodeType::DecodeWithLength => {
                        Ok(quote_owned_decode_with_length(input, &fields))
                    }
                },
                DecodeImplementation::Borrowed => match decode_type {
                    DecodeType::Decode => Ok(quote_borrowed_decode(input, &fields)),
                    DecodeType::DecodeWithLength => {
                        Ok(quote_borrowed_decode_with_length(input, &fields))
                    }
                },
                DecodeImplementation::All => match decode_type {
                    DecodeType::Decode => {
                        let quote_borrowed_decode = quote_borrowed_decode(input, &fields);
                        let quote_owned_decode = quote_owned_decode(input, &fields);

                        Ok(quote! {
                            #quote_borrowed_decode
                            #quote_owned_decode
                        })
                    }
                    DecodeType::DecodeWithLength => {
                        let quote_borrowed_decode =
                            quote_borrowed_decode_with_length(input, &fields);
                        let quote_owned_decode = quote_owned_decode_with_length(input, &fields);

                        Ok(quote! {
                            #quote_borrowed_decode
                            #quote_owned_decode
                        })
                    }
                },
            }
        }
    }
}

// XXX: Duplicated code with quote_owned_decode
fn quote_borrowed_decode(input: &DeriveInput, fields: &ValidFields) -> TokenStream {
    let name = &input.ident;

    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    // If there are no lifetimes, add <'a> to align with Decode<'a>
    let impl_generics = if input.generics.lifetimes().count() == 0 {
        let mut generics = input.generics.clone();
        generics.params.insert(
            0,
            syn::GenericParam::Lifetime(syn::LifetimeParam::new(syn::Lifetime::new(
                "'a",
                Span::call_site(),
            ))),
        );
        generics.split_for_impl().0.to_token_stream()
    } else {
        input.generics.split_for_impl().0.to_token_stream()
    };

    let skipped_field_exists = fields.fields.iter().any(|f| f.attrs.skip());
    let fields_names = fields.fields.iter().filter(|f| !f.attrs.skip()).map(|f| {
        f.field
            .ident
            .as_ref()
            .expect("Named fields must have idents")
    });

    let fields = fields.fields.iter().map(|f| f.quote_borrowed_decode());

    let constructor = if skipped_field_exists {
        quote! {
            Self::new(
                #(#fields_names),*
            )
        }
    } else {
        quote! {
            Self {
                #(#fields_names),*
            }
        }
    };

    quote! {
        impl #impl_generics crate::decode::borrowed::Decode<'a> for #name #ty_generics #where_clause {
            fn decode(src: &'a [u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
                let size = 0;
                #(
                    #fields
                )*

                Ok((#constructor, size))
            }
        }
    }
}

fn quote_owned_decode(input: &DeriveInput, fields: &ValidFields) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let skipped_field_exists = fields.fields.iter().any(|f| f.attrs.skip());
    let fields_names = fields.fields.iter().filter(|f| !f.attrs.skip()).map(|f| {
        f.field
            .ident
            .as_ref()
            .expect("Named fields must have idents")
    });

    let fields = fields.fields.iter().map(|f| f.quote_owned_decode());

    let constructor = if skipped_field_exists {
        quote! {
            Self::new(
                #(#fields_names),*
            )
        }
    } else {
        quote! {
            Self {
                #(#fields_names),*
            }
        }
    };

    quote! {
        #[cfg(feature = "alloc")]
        impl #impl_generics crate::decode::owned::Decode for #name #ty_generics #where_clause {
            fn decode(src: &mut ::bytes::BytesMut) -> Result<(Self, usize), crate::decode::DecodeError> {
                let size = 0;
                #(
                    #fields
                )*

                Ok((#constructor, size))
            }
        }
    }
}

// XXX: Skipped fields are not used here
fn quote_borrowed_decode_with_length(input: &DeriveInput, fields: &ValidFields) -> TokenStream {
    let name = &input.ident;

    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    // If there are no lifetimes, add <'a> to align with Decode<'a>
    let impl_generics = if input.generics.lifetimes().count() == 0 {
        let mut generics = input.generics.clone();
        generics.params.insert(
            0,
            syn::GenericParam::Lifetime(syn::LifetimeParam::new(syn::Lifetime::new(
                "'a",
                Span::call_site(),
            ))),
        );
        generics.split_for_impl().0.to_token_stream()
    } else {
        input.generics.split_for_impl().0.to_token_stream()
    };

    let fields_names = fields.fields.iter().filter(|f| !f.attrs.skip()).map(|f| {
        f.field
            .ident
            .as_ref()
            .expect("Named fields must have idents")
    });

    let fields = fields.fields.iter().map(|f| f.quote_borrowed_decode());

    quote! {
        impl #impl_generics crate::decode::borrowed::DecodeWithLength<'a> for #name #ty_generics #where_clause {
            fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), crate::decode::DecodeError> {
                let size = 0;
                #(
                    #fields
                )*

                Ok((Self {
                    #(#fields_names),*
                 }, size))
            }
        }
    }
}

// XXX: Skipped fields are not used here
fn quote_owned_decode_with_length(input: &DeriveInput, fields: &ValidFields) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let fields_names = fields.fields.iter().filter(|f| !f.attrs.skip()).map(|f| {
        f.field
            .ident
            .as_ref()
            .expect("Named fields must have idents")
    });

    let fields = fields.fields.iter().map(|f| f.quote_owned_decode());

    quote! {
        #[cfg(feature = "alloc")]
        impl #impl_generics crate::decode::owned::DecodeWithLength for #name #ty_generics #where_clause {
            fn decode(src: &mut ::bytes::BytesMut, length: usize) -> Result<(Self, usize), crate::decode::DecodeError> {
                let size = 0;
                #(
                    #fields
                )*

                Ok((Self {
                    #(#fields_names),*
                 }, size))
            }
        }
    }
}

struct StructAttributes {
    /// `#[rusmpp(repr = "u8")]`
    repr: Option<Repr>,
    decode_attrs: DecodeAttributes,
    test_attrs: TestAttributes,
}

impl StructAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut decode_attrs = DecodeAttributes::default();
        let mut test_attrs = TestAttributes::default();

        for attr in &input.attrs {
            if !attr.path().is_ident("rusmpp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("repr") {
                    let lit: Lit = meta.value()?.parse()?;

                    match lit {
                        Lit::Str(s) => match s.value().as_str() {
                            "u8" => repr = Some(Repr::new(ReprType::U8)),
                            "u16" => repr = Some(Repr::new(ReprType::U16)),
                            "u32" => repr = Some(Repr::new(ReprType::U32)),
                            other => {
                                return Err(meta.error(format!(
                                    "unsupported repr: {}, only `u8`, `u16`, and `u32` are allowed",
                                    other
                                )));
                            }
                        },
                        _ => {
                            return Err(meta.error("repr must be a string literal like \"u8\""));
                        }
                    }
                } else if meta.path.is_ident("decode") {
                    decode_attrs = DecodeAttributes::extract(meta)?;
                } else if meta.path.is_ident("test") {
                    test_attrs = TestAttributes::extract(meta)?;
                }

                Ok(())
            })?;
        }

        Ok(Self {
            repr,
            decode_attrs,
            test_attrs,
        })
    }
}

struct FieldAttributes {
    skip_decode: bool,
    length: Option<Length>,
    key: Option<Ident>,
    count: Option<Ident>,
}

impl FieldAttributes {
    fn extract(field: &Field) -> syn::Result<Self> {
        let mut skip_decode = false;
        let mut length = None;
        let mut key = None;
        let mut count = None;

        for attr in &field.attrs {
            if !attr.path().is_ident("rusmpp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip_decode") {
                    skip_decode = true;
                } else if meta.path.is_ident("length") {
                    let value = meta.value()?;

                    if let Ok(lit) = value.parse() {
                        if let Lit::Str(s) = lit {
                            match s.value().as_str() {
                                "unchecked" => length = Some(Length::Unchecked),
                                "checked" => length = Some(Length::Checked),
                                _ => {
                                    return Err(meta.error(
                                    "length must be \"unchecked\", \"checked\", or an identifier",
                                ));
                                }
                            }
                        }
                    }

                    if length.is_none() {
                        let ident: Ident = value.parse()?;
                        length = Some(Length::Ident(ident));
                    }
                } else if meta.path.is_ident("key") {
                    let ident: Ident = meta.value()?.parse()?;
                    key = Some(ident);
                } else if meta.path.is_ident("count") {
                    let ident: Ident = meta.value()?.parse()?;
                    count = Some(ident);
                } else {
                    return Err(meta.error("unknown attribute"));
                }

                Ok(())
            })?;
        }

        Ok(Self {
            skip_decode,
            length,
            key,
            count,
        })
    }

    fn validated(self) -> syn::Result<ValidFieldAttributes> {
        let Self {
            skip_decode,
            length,
            key,
            count,
        } = self;

        if skip_decode {
            if length.is_some() || key.is_some() || count.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "skip_decode cannot be combined with length, key, or count",
                ));
            }

            return Ok(ValidFieldAttributes::SkipDecode);
        }

        match (length, key, count) {
            (Some(Length::Unchecked), None, None) => Ok(ValidFieldAttributes::LengthUnchecked),
            (Some(Length::Checked), None, None) => Ok(ValidFieldAttributes::LengthChecked),
            (Some(Length::Ident(length)), None, None) => Ok(ValidFieldAttributes::LengthIdent {
                length_ident: length,
            }),
            (Some(Length::Unchecked), Some(key), None) => {
                Ok(ValidFieldAttributes::KeyLengthUnchecked { key_ident: key })
            }
            (Some(Length::Ident(length)), Some(key), None) => {
                Ok(ValidFieldAttributes::KeyLengthIdent {
                    key_ident: key,
                    length_ident: length,
                })
            }
            (None, None, Some(count)) => Ok(ValidFieldAttributes::Count { count_ident: count }),
            (None, None, None) => Ok(ValidFieldAttributes::None),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "invalid combination of field attributes",
            )),
        }
    }
}

enum Length {
    Unchecked,
    Checked,
    Ident(Ident),
}

enum ValidFieldAttributes {
    None,
    /// `#[rusmpp(skip_decode)]`
    SkipDecode,
    /// `#[rusmpp(length = "unchecked")]`
    LengthUnchecked,
    /// `#[rusmpp(length = "checked")]`
    LengthChecked,
    /// `#[rusmpp(length = ident)]`
    LengthIdent {
        length_ident: Ident,
    },
    /// `#[rusmpp(key = ident, length = "unchecked")]`
    KeyLengthUnchecked {
        key_ident: Ident,
    },
    /// `#[rusmpp(key = ident, length = ident)]`
    KeyLengthIdent {
        key_ident: Ident,
        length_ident: Ident,
    },
    /// `#[rusmpp(count = ident)]`
    Count {
        count_ident: Ident,
    },
}

impl ValidFieldAttributes {
    const fn requires_decode_with_length(&self) -> bool {
        matches!(
            self,
            Self::LengthUnchecked
                | Self::LengthChecked
                | Self::LengthIdent { .. }
                | Self::KeyLengthUnchecked { .. }
                | Self::Count { .. }
        )
    }

    const fn skip(&self) -> bool {
        matches!(self, Self::SkipDecode)
    }
}

struct ValidField<'a> {
    field: &'a Field,
    attrs: ValidFieldAttributes,
}

impl ValidField<'_> {
    fn quote_borrowed_decode(&self) -> TokenStream {
        let name = self
            .field
            .ident
            .as_ref()
            .expect("Named fields must have idents");

        match &self.attrs {
            ValidFieldAttributes::None => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(
                    crate::decode::borrowed::DecodeExt::decode_move(src, size),
                    crate::fields::SmppField::#name,
                )?;
            },
            ValidFieldAttributes::SkipDecode => quote! {},
            ValidFieldAttributes::LengthUnchecked => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::borrowed::DecodeWithLengthExt::decode_move(
                    src, length.saturating_sub(size), size
                ),crate::fields::SmppField::#name)?;
            },
            ValidFieldAttributes::LengthChecked => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::borrowed::DecodeExt::length_checked_decode_move(
                    src, length.saturating_sub(size), size
                ),crate::fields::SmppField::#name)?
                .map(|(this, size)| (Some(this), size))
                .unwrap_or((None, size));
            },
            ValidFieldAttributes::LengthIdent { length_ident } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::borrowed::DecodeWithLengthExt::decode_move(
                    src, #length_ident as usize, size
                ),crate::fields::SmppField::#name)?;
            },
            ValidFieldAttributes::KeyLengthUnchecked { key_ident } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::borrowed::DecodeWithKeyOptionalExt::decode_move(
                    #key_ident, src, length.saturating_sub(size), size
                ),crate::fields::SmppField::#name)?
                .map(|(this, size)| (Some(this), size))
                .unwrap_or((None, size));
            },
            ValidFieldAttributes::KeyLengthIdent {
                key_ident,
                length_ident,
            } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::borrowed::DecodeWithKeyExt::optional_length_checked_decode_move(
                    #key_ident, src, #length_ident as usize, size
                ),crate::fields::SmppField::#name)?
                .map(|(this, size)| (Some(this), size))
                .unwrap_or((None, size));
            },
            ValidFieldAttributes::Count { count_ident } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::borrowed::DecodeExt::counted_move(
                    src, #count_ident as usize, size
                ),crate::fields::SmppField::#name)?;
            },
        }
    }

    fn quote_owned_decode(&self) -> TokenStream {
        let name = self
            .field
            .ident
            .as_ref()
            .expect("Named fields must have idents");

        match &self.attrs {
            ValidFieldAttributes::None => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(
                    crate::decode::owned::DecodeExt::decode_move(src, size),
                    crate::fields::SmppField::#name,
                )?;
            },
            ValidFieldAttributes::SkipDecode => quote! {},
            ValidFieldAttributes::LengthUnchecked => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::owned::DecodeWithLengthExt::decode_move(
                    src, length.saturating_sub(size), size
                ),crate::fields::SmppField::#name)?;
            },
            ValidFieldAttributes::LengthChecked => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::owned::DecodeExt::length_checked_decode_move(
                    src, length.saturating_sub(size), size
                ),crate::fields::SmppField::#name)?
                .map(|(this, size)| (Some(this), size))
                .unwrap_or((None, size));
            },
            ValidFieldAttributes::LengthIdent { length_ident } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::owned::DecodeWithLengthExt::decode_move(
                    src, #length_ident as usize, size
                ),crate::fields::SmppField::#name)?;
            },
            ValidFieldAttributes::KeyLengthUnchecked { key_ident } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::owned::DecodeWithKeyOptionalExt::decode_move(
                    #key_ident, src, length.saturating_sub(size), size
                ),crate::fields::SmppField::#name)?
                .map(|(this, size)| (Some(this), size))
                .unwrap_or((None, size));
            },
            ValidFieldAttributes::KeyLengthIdent {
                key_ident,
                length_ident,
            } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::owned::DecodeWithKeyExt::optional_length_checked_decode_move(
                    #key_ident, src, #length_ident as usize, size
                ),crate::fields::SmppField::#name)?
                .map(|(this, size)| (Some(this), size))
                .unwrap_or((None, size));
            },
            ValidFieldAttributes::Count { count_ident } => quote! {
                let (#name, size) = crate::decode::DecodeErrorExt::map_as_source(crate::decode::owned::DecodeExt::counted_move(
                    src, #count_ident as usize, size
                ),crate::fields::SmppField::#name)?;
            },
        }
    }
}

struct ValidFields<'a> {
    fields: Vec<ValidField<'a>>,
}

impl ValidFields<'_> {
    /// Depending on the attributes, determine which decode impl to generate.
    #[allow(clippy::obfuscated_if_else)]
    fn decode_type(&self) -> DecodeType {
        self.fields
            .iter()
            .any(|f| f.attrs.requires_decode_with_length())
            .then_some(DecodeType::DecodeWithLength)
            .unwrap_or(DecodeType::Decode)
    }
}

impl<'a> From<Vec<ValidField<'a>>> for ValidFields<'a> {
    fn from(fields: Vec<ValidField<'a>>) -> Self {
        Self { fields }
    }
}

enum DecodeType {
    Decode,
    DecodeWithLength,
}

fn quote_test(input: &DeriveInput, test_attrs: &TestAttributes) -> TokenStream {
    match test_attrs {
        TestAttributes::Skip => quote! {},
        TestAttributes::Implement => {
            let name = &input.ident;
            let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

            quote! {
                #[cfg(test)]
                impl #impl_generics crate::tests::TestInstance for #name #ty_generics #where_clause {
                    fn instances() -> alloc::vec::Vec<Self> {
                        alloc::vec![Self::default(),]
                    }
                }
            }
        }
    }
}
