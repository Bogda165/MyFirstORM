use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DataStruct, DeriveInput, Expr, Field, Meta};
use syn::__private::TokenStream2;
use syn::Expr::Lit;
use syn::Lit::Str;
use syn::parse::Parser;
use crate::additional_functions::docs_manipulations::from_attribute_to_comment;

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
            impl #generics OrmTable for #table_name #generics
                #where_clause
            {
                type ColumnsT = #tuple;
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


pub fn get_tuple_from_table(mut table: DataStruct, _attrs_name: &str) -> TokenStream2{
    let types = &mut table.fields.iter_mut().filter_map(|field| {
        let _res = iter_through_attrs(field, false, |field, attr_name, _| {
            if attr_name == _attrs_name  {
                let field_name = field.clone().ident.unwrap();
                Some(quote!{<#field_name as TheType>::Type})
            }else {None}
        });

        if _res.len() == 0 {
            None
        }else {
            Some(_res[0].clone())
        }
    });

    quote!{
        (#(#types), *)
    }
}

pub fn attrs_to_comments_f(table: &mut DeriveInput){
    let new_fields = match &mut table.data {
        Data::Struct(_table) => {
            _table.fields.iter_mut().for_each(|field| {
                field.attrs.iter_mut().for_each(|attr| {
                    let new_attr = from_attribute_to_comment(attr.clone());
                    let parser = Attribute::parse_outer;
                    let new_attr_attr = parser.parse2(new_attr).unwrap_or_else(|_| vec![]);
                    *attr = new_attr_attr[0].clone()
                });
            })
        }
        Data::Enum(_) => {unreachable!()}
        Data::Union(_) => {unreachable!()}
    };
}