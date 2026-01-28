use crate::ty_to_rust_type_is_wincode;
use anchor_lang_idl_spec::{IdlDefinedFields, IdlField, IdlType};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates struct fields from a list of [IdlField]s.
pub fn generate_struct_fields_from_slice(fields: &[IdlField]) -> TokenStream {
    let fields_rendered = fields.iter().map(|arg| {
        let name = format_ident!("{}", arg.name.to_snake_case());
        let type_name = crate::ty_to_rust_type(&arg.ty);
        let stream: proc_macro2::TokenStream = type_name.parse().unwrap();
        match &arg.ty {
            IdlType::Bytes => {
                quote! {
                    #[wincode(with = "wincode::containers::Vec<u8, U32SeqLen>")]
                    pub #name: #stream
                }
            }
            IdlType::Vec(inner) => {
                let wincode_path_str = format!(
                    "wincode::containers::Vec<{}, U32SeqLen>",
                    ty_to_rust_type_is_wincode(inner, true)
                );
                let wincode_path_lit =
                    syn::LitStr::new(&wincode_path_str, proc_macro2::Span::call_site());
                let derive = quote! { #[wincode(with = #wincode_path_lit)] };
                quote! {
                    #derive
                    pub #name: #stream
                }
            }
            _ => {
                quote! {
                    pub #name: #stream
                }
            }
        }
    });
    quote! {
        #(#fields_rendered),*
    }
}

pub fn get_idl_defined_fields_as_slice(fields: &Option<IdlDefinedFields>) -> &[IdlField] {
    match fields {
        Some(IdlDefinedFields::Named(fields)) => fields,
        None => &[],
        _ => todo!(),
    }
}

/// Generates struct fields from a list of [IdlField]s.
pub fn generate_struct_fields(fields: &Option<IdlDefinedFields>) -> TokenStream {
    if let Some(fields) = fields {
        match fields {
            IdlDefinedFields::Named(fields) => generate_struct_fields_from_slice(fields),
            IdlDefinedFields::Tuple(_) => todo!(),
        }
    } else {
        quote! {}
    }
}
