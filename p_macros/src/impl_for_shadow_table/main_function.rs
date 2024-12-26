use proc_macro::TokenStream;
use std::collections::HashMap;
use proc_macro2::{Ident};
use quote::quote;
use syn::{Attribute, DeriveInput, Type};
use Db_shit::{Attributes, DbTypes};
use crate::additional_functions::attributes_manipulations::{create_attr_with_type, to_string};
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


fn from_row(shadow_t_ident: &Ident, construct_table_s: &HashMap<Ident, Vec<Attribute>>) -> proc_macro2::TokenStream {

    let inside = construct_table_s.iter().map(|field| {
        let name = field.0;
        let attrs = field.1;

        if attrs.len() <= 0 {
            return quote!{};
        }

        let mut id = -1;

        let attributes_parsed = attrs.iter().map(|attr| {
            id += 1;
            let inside_dv_type_fn = |input: proc_macro2::TokenStream| -> proc_macro2::TokenStream{
                let all_types: HashMap<String, Type> = DbTypes::get_types_nt();

                let _type = all_types.get(&attr.meta.path().get_ident().unwrap().to_string()).unwrap();
                let _input_str = input.to_string();


                quote! {
                    row.get::<&str, #_type>(#_input_str).unwrap()
                }
            };

            match create_attr_with_type(attr, name, inside_dv_type_fn) {
                Ok(attr) => {attr}
                Err(_) => {
                    std::panic!("Not allowed type or attr")
                }
            }

        });

        quote! {
            #name: (
                #(#attributes_parsed),*
            )
        }
    });

    quote! {
         pub fn from_row(row: &Row) -> Self {
            #shadow_t_ident {
                #(#inside),*
            }
        }
    }
}

pub fn generate_function(input: &DeriveInput, construct_table_s: HashMap<Ident, Vec<Attribute>>, table_name_ident: &Ident) -> TokenStream{
    let meta_data = MetaData::default();

    //create an impl for table
    let create_function = for_create_function(table_name_ident, &construct_table_s, &meta_data);
    eprintln!("create method created");

    let insert_function = insert_function(table_name_ident, &construct_table_s);

    let load_function = load_function(table_name_ident);

    let from_row_function = from_row(table_name_ident, &construct_table_s);

    TokenStream::from(quote! {
        use rusqlite::Row;
        #input
        impl #table_name_ident{

            #create_function

            #insert_function

            #load_function

            #from_row_function
        }
    })
}