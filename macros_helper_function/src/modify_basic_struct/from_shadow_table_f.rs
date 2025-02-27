use std::collections::HashMap;
use proc_macro2::Ident;
use quote::quote;
use syn::Attribute;

pub fn from_shadow_table_f(shadow_t_ident:&Ident, table_ident: &Ident, construct_table_s: &HashMap<Ident, Vec<Attribute>>) -> proc_macro2::TokenStream {

    let inside = construct_table_s.iter().map(|field| {
        let name = field.0;
        let attrs = field.1;

        // if the field shouldn't be in in shadow table
        if attrs.len() == 0 {
            return quote!{};
        }
        // get data_type index
        let index = 0;

        eprintln!("Construct table: ");
        attrs.iter().for_each(|attr| {
            eprintln!("{}", attr.meta.path().get_ident().unwrap().to_string());
        });

        let data_type_i = {
            let mut tmp = quote! {
                shadow_table.#name
            };
            if attrs.len() > 1 {
                tmp = quote! {
                     shadow_table.#name.#index
                }
            }
            tmp
        };

        let data_type_t = attrs[index].meta.path().get_ident().unwrap();

        quote! {
            #shadow_t_ident.#name = match #data_type_i {
                DbTypes::#data_type_t(val) => {
                    val
                },
                _ => panic!("Incorrect type, while parsing from shadow table")
            };

        }
    });

    quote!{
        pub fn from_shadow_table(shadow_table: #shadow_t_ident) -> Self {
            let mut #shadow_t_ident = #table_ident::default();
            #(#inside)*
            #shadow_t_ident
        }
    }
}