use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, FieldsNamed, Ident};

pub fn quote_parts(input: &DeriveInput, fields_named: &FieldsNamed) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let parts_struct_name = Ident::new(&format!("{}Parts", name), name.span());

    let parts_struct_field_names_and_types = fields_named.named.iter().map(|f| {
        let ident = f.ident.as_ref().expect("Named fields must have idents");
        let ty = &f.ty;

        (ident, ty)
    });

    let parts_struct_fields = parts_struct_field_names_and_types
        .clone()
        .map(|(ident, ty)| quote! { pub #ident: #ty });

    let parts_struct_new_parameters = parts_struct_field_names_and_types
        .clone()
        .map(|(ident, ty)| quote! { #ident: #ty });

    let parts_struct_field_names = parts_struct_field_names_and_types
        .clone()
        .map(|(ident, _)| ident);

    let parts_struct_field_names_clone = parts_struct_field_names.clone();
    let parts_struct_field_names_clone_2 = parts_struct_field_names.clone();

    let parts_struct_field_types = parts_struct_field_names_and_types.clone().map(|(_, ty)| ty);

    let parts_struct_field_names_self_names = parts_struct_field_names
        .clone()
        .map(|ident| quote! { #ident: self.#ident });

    quote! {
        #[derive(Debug)]
        pub struct #parts_struct_name #generics {
            #(#parts_struct_fields),*
        }

        impl #impl_generics #parts_struct_name #ty_generics #where_clause {
            #[inline]
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                #(#parts_struct_new_parameters),*
            ) -> Self {
                Self {
                    #(#parts_struct_field_names),*
                }
            }

            #[inline]
            #[allow(unused_parens)]
            pub fn raw(self) -> (#(#parts_struct_field_types),*) {
                (#(self.#parts_struct_field_names_clone),*)
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            /// Converts [`Self`] into its parts.
            #[inline]
            pub fn into_parts(self) -> #parts_struct_name #ty_generics #where_clause {
                #parts_struct_name {
                    #(#parts_struct_field_names_self_names),*
                }
            }

            /// Creates a new instance of [`Self`] from its parts.
            ///
            /// # Note
            ///
            /// This may create invalid instances. It's up to the caller to ensure that the parts are valid.
            #[inline]
            pub fn from_parts(parts: #parts_struct_name #ty_generics) -> Self #where_clause {
                Self {
                    #(#parts_struct_field_names_clone_2: parts.#parts_struct_field_names_clone_2),*
                }
            }
        }
    }
}
