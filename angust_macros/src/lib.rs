extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields, FieldsNamed, ItemStruct, Type, Ident};

#[proc_macro_derive(ReflectiveStruct)]
pub fn reflective_struct_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Extract fields from the struct
    let fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("ReflectiveStruct can only be used on structs"),
    };

    // Implement getter for fields
    let get_field_arms = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name_str => Some(&self.#field_name),
        }
    });

    // Implement setter for fields
    let set_field_arms = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name_str => {
                if let Ok(value) = value.downcast::<#field_type>() {
                    self.#field_name = *value;
                } else {
                    panic!("Type mismatch for field '{}'", #field_name_str);
                }
            }
        }
    });

    // Implement listing all field names
    let get_all_properties_arms = fields.iter().map(|field| {
        let field_name_str = field.ident.as_ref().unwrap().to_string();
        quote! {
            properties.push(#field_name_str);
        }
    });

    // Generate the full implementation
    let expanded = quote! {
        impl Reflect for #struct_name {
            fn get_field(&self, name: &str) -> Option<&dyn Reflect> {
                match name {
                    #(#get_field_arms)*
                    _ => None,
                }
            }

            fn set_field(&mut self, name: &str, value: Box<dyn std::any::Any>) {
                match name {
                    #(#set_field_arms)*
                    _ => panic!("Field '{}' not found", name),
                }
            }

            fn get_all_properties(&self) -> Vec<&str> {
                let mut properties = Vec::new();
                #(#get_all_properties_arms)*
                properties
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}


// Not functional yet
#[proc_macro_attribute]
pub fn reactive_struct(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_struct = parse_macro_input!(input as ItemStruct);

    // Only proceed if the struct has named fields
    if let Fields::Named(ref mut fields) = input_struct.fields {
        for field in fields.named.iter_mut() {
            let field_type = &field.ty;
            // Wrap each field type with `ReactiveField`
            field.ty = syn::parse(quote!(ReactiveField<#field_type>).into()).unwrap();
        }
    } else {
        panic!("reactive_struct can only be used with structs having named fields");
    }

    let struct_name = &input_struct.ident;
    let fields_initializers = input_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            #field_name: ReactiveField::new(Default::default()),
        }
    });

    let subscribe_methods = input_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            stringify!(#field_name) => self.#field_name.subscribe(callback),
        }
    });

    // Output the transformed struct with additional ReactiveState implementation
    let expanded = quote! {
        #input_struct

        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(#fields_initializers)*
                }
            }
        }

        impl ReactiveState for #struct_name {
            fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
            where F: 'static + FnMut(&ComponentEvent) {
                match property_name {
                    #(#subscribe_methods)*,
                    _ => {} // Handle case where property name does not match
                }
            }
        }
    };

    TokenStream::from(expanded)
}