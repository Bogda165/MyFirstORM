use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Data, DataStruct, DeriveInput, Error, Meta, Path, Token};
use crate::additional_functions::functions::iter_through_attrs;

pub fn impl_relations_func(input: DeriveInput) -> TokenStream2 {
    let name = input.clone().ident;
    let mut table = if let Data::Struct(table) = input.clone().data {
        table
    }else {
        std::panic!("OrmTable must be implemented only ofr structs")
    };

    let generics = input.generics;

    let where_clause = if let Some(where_clause) = generics.clone().where_clause {
        quote!{#where_clause}
    }else {
        quote!{}
    };

    let mut result = vec![];
    table.fields.iter_mut().map(|field| {
        ///TODO create a parser insetad of this piece of shit
        let mut table_name: Option<Path> = None;
        let mut connect_column: Option<Path> = None;
        let mut relation_type: Option<Ident> = None;
        let self_ident: Option<Ident> = None;
        let _type: Option<Ident> = None;
        iter_through_attrs(field, false, |field, attrs_name, attr| {
            match &*attrs_name {
                "relation" => {
                    match attr.meta {
                        Meta::Path(_) => {}
                        Meta::List(ref listed_values) => {
                            let pathes_vec = syn::parse::<crate::custom_parser::CommaPath>(TokenStream::from(listed_values.clone().tokens)).unwrap().into();
                        }
                        Meta::NameValue(_) => {}
                    }
                }
                _ => {
                    None
                }
            }
        })
    }).for_each(|token_stream_vec| {
        result.extend(token_stream_vec)
    });

    quote!{
        #(#result)*
    }
}