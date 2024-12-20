use std::collections::HashMap;
use proc_macro2::Ident;
use quote::quote;
use syn::Attribute;
use crate::additional_functions::attributes_manipulations::create_attr_with_type;

pub fn get_shadow_table(construct_table_s: &HashMap<Ident, Vec<Attribute>>, shadow_t_name: &Ident, table_name: &Ident) -> proc_macro2::TokenStream {
    let fields = construct_table_s.iter().map(|field|{
        if field.1.len() == 0 {
            return quote!{};
        }
        let name = field.0;
        let attrs = field.1.iter().map(|attr| {
            match create_attr_with_type(attr, name.clone()) {
                Ok(attr) => {attr}
                Err(_) => {
                    std::panic!("Not allowed type or attr")
                }
            }
        });
        quote! {
            #name: (#(#attrs),* ),
        }
    });
    let shadow_t_instance = quote!(
        #[doc = "I work here"]
        #shadow_t_name
        {
            #(#fields)*
        }
    );

    quote! {
        impl #table_name {
            pub fn get_table2(&self) -> #shadow_t_name {
                #shadow_t_instance
            }
        }
    }
}