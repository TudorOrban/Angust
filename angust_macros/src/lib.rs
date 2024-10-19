extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ReflectiveStruct)]
pub fn reflective_struct_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("ReflectiveStruct can only be used on structs"),
    };

    let get_field_arms = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name_str => Some(&self.#field_name),
        }
    });

    let expanded = quote! {
        impl Reflect for #struct_name {
            fn get_field(&self, name: &str) -> Option<&dyn Reflect> {
                match name {
                    #(#get_field_arms)*
                    _ => None,
                }
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}