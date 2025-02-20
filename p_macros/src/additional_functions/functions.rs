use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DeriveInput, Expr, Field, Meta};
use syn::__private::TokenStream2;
use syn::Expr::Lit;
use syn::Lit::Str;
use crate::additional_functions::attributes_manipulations::is_in_allowed_attrs;
use crate::{get_tuple_from_table, FroConnect};

pub fn handle_field_table_struct(field: (&Ident, &Vec<Attribute>)) -> proc_macro2::TokenStream {
    let name = field.0;
    //check if connect logic
    if field.1.len() == 0 {
        return quote!{};
    }
    if field.1[0].meta.path().get_ident().unwrap().to_string() == "Connect" {

        let mut connect_o = FroConnect::default();
        field.1.iter().for_each(|attr| {
            match attr.meta {
                Meta::Path(_) => {
                    std::panic!("Not expected attribute {}", attr.meta.path().get_ident().unwrap().to_string())
                }
                Meta::List(_) => {
                    std::panic!("Not expected attribute {}", attr.meta.path().get_ident().unwrap().to_string())
                }
                Meta::NameValue(ref attr) => {
                    match &*attr.path.get_ident().unwrap().to_string() {
                        "path" => {
                            if let Lit(expr_list) = &attr.value {
                                if let Str(lit_str) = &expr_list.lit {
                                    connect_o.path = lit_str.value()
                                }
                            }
                        }
                        "table_name" => {
                            if let Lit(expr_list) = &attr.value {
                                if let Str(lit_str) = &expr_list.lit {
                                    connect_o.table_name = lit_str.value()
                                }
                            }
                        }
                        "field_name" => {
                            if let Lit(expr_list) = &attr.value {
                                if let Str(lit_str) = &expr_list.lit {
                                    connect_o.field_name = lit_str.value()
                                }
                            }
                        }
                        _ => {
                            std::panic!("Not expected attribute name")
                        }
                    }
                }
            }
        });


        /*
        //find struct with name table_name, and get the type of field_name
        //form quote
        let mut field_type;
        //find struct
        match find_by_name(&*connect_o.path, connect_o.table_name.clone()) {
            Ok(table) => {
                assert_eq!(table.ident.to_string(), connect_o.table_name);
                if let Fields::Named(fields) = table.fields {
                    for field in fields.named {
                        if field.ident.unwrap().to_string() == connect_o.field_name {
                            field_type = field.ty;
                        }
                    }
                }

            }
            Err(err) => {
                panic!("Couldn't find strcut with name {}; error {:?}", connect_o.table_name, err);
            }
        };

         */

        //get Dbtype from file_type
    }

    let attrs = field.1.iter().map(|attr| {
        match is_in_allowed_attrs(&attr.meta.path().get_ident().unwrap()) {
            Ok(_attr) => {
                _attr
            }
            Err(_) => {
                std::panic!("Not allowed type or attr")
            }
        }
    });

    quote! {
        pub #name: (#(#attrs),* ),
    }
}


/// if the F func the None if the attribute wasn't find, and Some(index, TokenStream) if was
pub fn iter_through_attrs<T: ToTokens, MatchF>(field: &mut Field, delete_attrs: bool, mut func: MatchF) -> Vec<T>
where
    MatchF: FnMut(&Field, String, Attribute) -> Option<T>,
{
    let attrs_amount = field.attrs.len();
    let mut remove_attrs: Vec<usize> = vec![];

    let opened_attrs= field.attrs.iter().zip(0..attrs_amount).filter_map(|(attr, index)| {
        match attr.meta.path().get_ident() {
            None => { None }
            Some(attr_ident) => {
                let token_stream =  func(field, attr_ident.to_string(), attr.clone());
                if let Some(ref ts)  = token_stream {
                    remove_attrs.push(index);
                }
                token_stream
            }
        }
    });

    let _result = opened_attrs.collect();

    if delete_attrs {
        eprintln!("Removing attrs: {:?}", remove_attrs);
        let length = remove_attrs.len();
        remove_attrs.into_iter().zip(0..length).for_each(|(index, remove_i)| {
            field.attrs.remove(remove_i - index);
        });
    }

    _result
}

pub fn orm_table_derive_f(input: DeriveInput) -> TokenStream2 {
    let table_name = input.clone().ident;
    let mut table = if let Data::Struct(table) = input.clone().data {
        table
    }else {
        std::panic!("OrmTable must be implemented only ofr structs")
    };

    let inside_impl = crate::derive_orm_traits::orm_table::orm_table_derive_f(table_name.clone(), table.clone());
    let tuple = get_tuple_from_table(table.clone(), "column");

    let _final = {
        let generics = input.generics;

        let where_clause = if let Some(where_clause) = generics.clone().where_clause {
            quote!{#where_clause}
        }else {
            quote!{}
        };

        quote! {
            use orm_traits::OrmTable;
            impl #generics OrmTable<#tuple> for #table_name #generics
                #where_clause
            {
                #inside_impl
            }
        }
    };

    _final
}

pub fn get_inside_attrs<F>(field: &Field, attr_name: &str, inside_attr_check: F) -> Vec<Ident>
where
    F: Fn(&Ident) -> Result<&Ident, ()>
{
    field.attrs.iter().filter_map(|attr| {
        if let Some(ident) = attr.path().get_ident() {
            if &*ident.to_string() == attr_name {
                match &attr.meta {
                    Meta::Path(_) => {
                        dbg!("path_type");
                        panic!("No constraints were listed")
                    }
                    Meta::List(constraints) => {
                        dbg!("list_type");
                        let mut _result = vec![];

                        constraints.parse_nested_meta(|meta| {
                            match inside_attr_check(meta.path.get_ident().unwrap()) {
                                Ok(ident) => {
                                    _result.push(ident.clone());
                                }
                                _ => { panic!("Unknown constraint") }
                            }
                            Ok(())
                        }).expect("TODO: panic message");

                        Some(_result)
                    }
                    Meta::NameValue(val) => {
                        dbg!("name_value");
                        if let Expr::Path(constraint_name) = &val.value {
                            Some(vec![constraint_name.path.get_ident().expect("Unknown expression for constraint").clone()])
                        }else {
                            panic!("Unknown expression for constraint")
                        }
                    }
                }
            }else {
                dbg!(&*ident.to_string());
                None
            }
        }else {
            None
        }
    }).fold(vec![], |mut vec, mut vec_ident| {
        vec.append(&mut vec_ident);
        vec
    })
}


fn to_real_type() {

}