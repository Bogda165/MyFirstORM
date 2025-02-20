use proc_macro2::{Ident};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Expr, Field, Meta, Path, PathSegment, Token, Type};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::additional_functions::functions::iter_through_attrs;
use crate::get_tuple_from_table;

fn columns_fn(table: &mut DataStruct) -> TokenStream2 {
    let columns_types = get_tuple_from_table(table.clone(), "column");

    let fields = table.clone().fields.into_iter().filter_map(|mut field| {
        let name = field.clone().ident.unwrap();
        let temp_vec = iter_through_attrs(&mut field, false, |field, attr_name, _| {
            match &*attr_name {
                "column" => {
                    Some(quote! {
                        #name
                    })
                }
                _ => None
            }
        });
        if temp_vec.len() == 0{
            None
        }else {
            Some(temp_vec[0].clone())
        }
    });

    quote!{
        fn columns(self) -> #columns_types {
            (#(self.#fields.into()),*)
        }
    }
}

fn is_correct_constraint(constraint: &Ident) -> Result<&Ident, ()> {
    Ok(constraint)
}

pub fn get_constraints<'a>(field: &Field) -> Vec<Ident> {
    crate::additional_functions::functions::get_inside_attrs(field, "constraint", is_correct_constraint)
}

fn columns_strings_fn(table: &mut DataStruct) -> TokenStream2 {
    table.fields.iter().for_each(|field| {
        eprintln!("Constraints: {:?}", get_constraints(field));
    });

    let columns = table.fields.iter_mut().filter_map(|field| {
        let temp_vec = iter_through_attrs(field, false, |field, attr_name, _| {
            match &*attr_name {
                "column" => {
                    let constraints = get_constraints(field);
                    let name = field.clone().ident.unwrap();
                    Some(quote! {
                        {
                            let mut column: OrmColumn = #name.into();
                            column.attrs = vec![#(#constraints.to_query()), *];
                            column
                        }
                    })
                }
                _ => None
            }
        });
        if temp_vec.len() == 0{
            None
        }else {
            Some(temp_vec[0].clone())
        }
    });

    quote! {
        fn columns_strings() -> Vec<OrmColumn> {
            vec![#(#columns), *]
        }
    }
}

fn from_columns_fn(table: &mut DataStruct, table_name: &Ident) -> TokenStream2 {
    let tuple = get_tuple_from_table(table.clone(), "column");
    let mut index = -1;

    //TODO handle situation when the sie of fields is 1
    let columns = table.fields.iter_mut().filter_map(|mut field| {
        let temp_vec =
            iter_through_attrs(field, false, |field, attr_name, _| {
                match &*attr_name {
                    "column" => {
                        eprintln!("column attr was found");
                        let name = field.clone().ident;
                        index += 1;
                        Some(quote!{
                            #name: columns.#index
                        })
                    }
                    _ => {
                        eprintln!("Attr name {}", attr_name);
                        None
                    }
                }
            });
        eprintln!("Tem vec: {:?}", temp_vec);
        eprintln!("Attributes amount: {}", field.attrs.len());
        if temp_vec.len() == 0{
            None
        }else {
            Some(temp_vec[0].clone())
        }
    });


    quote! {
        fn from_columns(columns: #tuple) -> Self
        where
            Self: Sized,
        {
            #table_name {
                #(#columns), *
                ,
                ..Default::default()
            }
        }
    }
}

pub fn orm_table_derive_f(table_name: Ident, mut table: DataStruct) -> TokenStream2{
    //let columns_types = get_tuple_from_table(&mut table, "column");

    let columns = columns_fn(&mut table);
    let columns_strings = columns_strings_fn(&mut table);
    let from_columns = from_columns_fn(&mut table, &table_name);

    quote! {
        #columns
        #columns_strings
        #from_columns
    }
}
