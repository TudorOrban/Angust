extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn component_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &input_struct.ident;

    let fields = if let Fields::Named(ref fields) = input_struct.fields {
        fields.named.iter().collect::<Vec<_>>()
    } else {
        panic!("component_state can only be applied to structs with named fields");
    };


    let reactive_field_definitions = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let reactive_field_name = format_ident!("{}_reactive", name);
        quote! {
            pub #reactive_field_name: ReactiveField<#ty>,
            pub #name: #ty,
        }
    });

    let constructor_params = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #name: #ty
        }
    });

    let field_initializers = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let reactive_field_name = format_ident!("{}_reactive", name);
        quote! {
            #reactive_field_name: ReactiveField::new(#name.clone()), // Clone values to avoid moves
            #name: #name,
        }
    });
    
    let get_field_arms = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name_str => Some(Box::new(self.#field_name.clone())),
        }
    });

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

    let get_all_properties_arms = fields.iter().map(|field| {
        let field_name_str = field.ident.as_ref().unwrap().to_string();
        quote! {
            properties.push(#field_name_str);
        }
    });

    let reactive_field_subscriptions = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let reactive_field_name = format_ident!("{}_reactive", field_name);
        quote! {
            stringify!(#field_name) => self.#reactive_field_name.subscribe(callback),
        }
    });

    // Generate the final expanded code
    let expanded = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(#reactive_field_definitions)*
        }

        impl #struct_name {
            // Constructor function
            pub fn new(#(#constructor_params),*) -> Self {
                Self {
                    #(#field_initializers)*
                }
            }
        }

        impl ReflectiveState for #struct_name {
            fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveState>> {
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

            fn as_any(&self) -> Box<dyn std::any::Any> {
                Box::new(self.clone())
            }

            fn clone_box(&self) -> Box<dyn ReflectiveState> {
                Box::new(self.clone())
            }
        }

        impl ReactiveState for #struct_name {
            fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
            where F: 'static + FnMut(&ComponentEvent) {
                match property_name {
                    #(#reactive_field_subscriptions)*
                    _ => {},
                }
            }
        }
    };

    TokenStream::from(expanded)
}
