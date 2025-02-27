use std::collections::HashMap;
use proc_macro2::{Ident};
use quote::quote;
use syn::{Attribute, DeriveInput, Type};
use syn::__private::TokenStream2;
use crate::additional_functions::attributes_manipulations::{to_string};
use crate::meta_data::MetaData;


fn for_create_function(table_name_ident: &Ident, construct_table_s: &HashMap<Ident, Vec<Attribute>>, meta_data: &MetaData) -> proc_macro2::TokenStream {
    let mut query = String::from(format!("CREATE TABLE {} (\n", table_name_ident.to_string()));
    construct_table_s.iter().for_each(|field| {
        query += &field.0.to_string();
        query += "\t";
        field.1.iter().for_each(|attr| {
            query += to_string(attr.meta.path().get_ident().unwrap(), meta_data.clone());
            query += " "
        });
        query += ",\n"
    });
    query.pop();
    query.pop();
    query += "\n);";

    quote!{
        pub fn create(& self) -> String{
            #query.to_string()
        }
    }
}

fn create_vals(construct_table_s: &HashMap<Ident, Vec<Attribute>>, size: &mut usize) -> proc_macro2::TokenStream {
    let vals = {
        let fields_vals = construct_table_s.iter().map(|field| {
            *size += 1;
            let name = field.0;
            let mut res = quote! {#name};
            if field.1.len() != 1 {
                res = quote! {#name.0};
            }

            quote! {
                #res
            }
        });
        quote! {
            #(&self.#fields_vals), *
        }
        //let mut query = String::from(format!("INSERT INTO {} ({}) VALUES \n", table_name_ident.to_string(), fields_names));
    };
    eprintln!("vals generated");

    vals
}

fn insert_function(table_name_ident: &Ident, construct_table_s: &HashMap<Ident, Vec<Attribute>>) -> proc_macro2::TokenStream {
    let mut size: usize = 0;

    let vals = create_vals(construct_table_s, &mut size);

    let mut question_marks = "".to_string();

    let paren = {
        let _return = (0..size).map(|_| {
            question_marks += "?, ";
            quote! {
                &DbTypes
            }
        });

        quote! {
            (#(#_return), *)
        }
    };
    question_marks.pop();
    question_marks.pop();
    eprintln!("question marks generated");

    let ident = {
        let mut string = "".to_string();

        construct_table_s.iter().for_each(|field| {
            string += &*field.0.to_string();
            string += " ,"
        });
        string.pop();
        string.pop();

        quote! {
            #string
        }
    };
    eprintln!("ident is generated");

    let ts = table_name_ident.to_string();
    eprintln!("{:?}", ident.to_string());

     quote! {
        pub fn insert(&self) -> (String, #paren){
            let query = format!("INSERT INTO {} ({}) VALUES ({});", #ts, #ident.to_string() , #question_marks);
            (query, (#vals))
        }
    }
}


fn load_function(table_name: &Ident) -> proc_macro2::TokenStream {
    let table_name_s = table_name.to_string();
    quote! {
        pub fn load(params: &str) -> String{
            format!("SELECT * from {0}\n{1}", #table_name_s, params)
        }
    }
}
