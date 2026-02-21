use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, FieldsNamed, Ident};

pub fn quote_decode_error(input: &DeriveInput, fields_named: &FieldsNamed) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let decode_error_struct_name = Ident::new(&format!("{}DecodeError", name), name.span());

    let decode_error_struct_field_names_and_types = fields_named.named.iter().map(|f| {
        let ident = f.ident.as_ref().expect("Named fields must have idents");
        let ty = &f.ty;

        (ident, ty)
    });

    let parts_struct_fields = decode_error_struct_field_names_and_types
        .clone()
        .map(|(ident, ty)| quote! { pub #ident: ::core::result::Result<#ty, <#ty as crate::decode::DecodeErrorType>::Error> });

    quote! {
        #[derive(Debug)]
        pub struct #decode_error_struct_name #generics {
            #(#parts_struct_fields),*
        }

        impl #impl_generics crate::decode::DecodeErrorType for #name #ty_generics #where_clause {
            type Error = #decode_error_struct_name #ty_generics;
        }
    }
}
