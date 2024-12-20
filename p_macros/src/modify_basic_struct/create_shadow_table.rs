use std::collections::HashMap;
use proc_macro2::Ident;
use quote::quote;
use syn::Attribute;
use crate::additional_functions::docs_manipulations::from_attribute_to_comment;
use crate::additional_functions::functions::handle_field_table_struct;

pub fn create_shadow_table(construct_table_s: &HashMap<Ident, Vec<Attribute>>, shadow_table_name: &Ident) -> proc_macro2::TokenStream {
    let construct_table_s = construct_table_s.clone();

    let fields = construct_table_s.iter().map(|field| {
        let _field = handle_field_table_struct(field);

        let attributes = field.1.iter().map(|attribute|{
            eprintln!("{:?}", attribute.meta.path().get_ident().unwrap().to_string());
            from_attribute_to_comment(attribute.clone())
        });

        quote! {
            #(#attributes)*
            #_field
        }
    });

    let _return = quote!{
        pub struct #shadow_table_name {
            #(#fields)*
        }
    };
    eprintln!("The structure is generated");
    _return
}