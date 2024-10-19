extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Fields, FieldsNamed};

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



#[proc_macro_attribute]
pub fn reactive_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut struct_ast = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &struct_ast.ident;

    // Collect field setups
    let fields = if let syn::Fields::Named(ref fields) = struct_ast.fields {
        fields.named.iter().collect::<Vec<_>>()
    } else {
        panic!("ReactiveState can only be applied to structs with named fields");
    };

    // Add ReactiveField properties and modify constructor
    let reactive_field_definitions = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        let reactive_field_name = format_ident!("{}_reactive", field_name.clone().unwrap());
        quote! {
            pub #reactive_field_name: ReactiveField<#field_type>,
        }
    });

    let reactive_field_initializations = fields.iter().map(|f| {
        let field_name = &f.ident;
        let reactive_field_name = format_ident!("{}_reactive", field_name.clone().unwrap());
        quote! {
            #reactive_field_name: ReactiveField::new(Default::default()),
        }
    });

    let reactive_field_subscriptions = fields.iter().map(|f| {
        let field_name = &f.ident;
        let reactive_field_name = format_ident!("{}_reactive", field_name.clone().unwrap());
        quote! {
            stringify!(#field_name) => self.#reactive_field_name.subscribe(callback),
        }
    });

    let expanded = quote! {
        // Modify the original struct to include reactive fields
        pub struct #struct_name {
            #(#fields),*
            #(#reactive_field_definitions)*
        }

        impl #struct_name {
            // Adjusted new function to initialize both normal and reactive fields
            pub fn new(#(#fields),*) -> Self {
                Self {
                    #(#fields),*
                    #(#reactive_field_initializations)*
                }
            }
        }

        // Implement ReactiveState for added reactive functionalities
        impl ReactiveState for #struct_name {
            fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
            where F: 'static + FnMut(&ComponentEvent) {
                match property_name {
                    #(#reactive_field_subscriptions,)*
                    _ => {} // handle default case
                }
            }
        }
    };

    TokenStream::from(expanded)
}