use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, LitStr, Token, Visibility};
use crate::additional_functions::construct_table::create_construct_table;

use crate::new_macros::table_def::impl_table;
use crate::additional_functions::functions::orm_table_derive_f;
use crate::additional_functions::functions::attrs_to_comments_f;
use crate::load_funcs::load_funcs;
use crate::relations::impl_relations_func;

pub fn create_macro(mut input: DeriveInput, shadow_table_name_i: Ident, name: Ident, shadow_table_name: LitStr) -> proc_macro2::TokenStream {
    let data = match input.data {
        Data::Struct(ref mut data) => {
            data
        }
        _ => {
            std::panic!("Not a structure");
        }
    };

    let table_name = input.ident.clone();

    let construct_table_s = create_construct_table(&data);

    let impl_table = impl_table((data, &name), false, quote!{});
    let impl_relations = impl_relations_func(input.clone());
    let impl_orm_table = orm_table_derive_f(input.clone());
    let impl_loading_part = load_funcs(input.clone());


    attrs_to_comments_f(&mut input);
    
    input.vis = Visibility::Public(Default::default());


    let shadow_table_name_s = shadow_table_name_i.to_string();
    //connect
    quote!{
        pub mod #shadow_table_name_i{
            use super::*;
            use dsl::column::Column;
            use dsl::column::RawColumn;
            use dsl::expressions::raw_types::RawTypes;
            use dsl::convertible::TheType;
            use dsl::column::Allowed;
            use dsl::column::Table;
            use rusqlite::types::{FromSql, ValueRef, FromSqlResult};

            //#[derive(Debug)]
            //#shadow_table

            //#[doc = #shadow_table_name_s]
            #input

            #impl_table
            #impl_orm_table
            #impl_loading_part
            #impl_relations

            impl FromSql for #table_name {
                fn column_result(value: ValueRef) -> FromSqlResult<Self> {
                    todo!()
                }
            }

            // impl Entity for #name
            // {
            //     fn get_table_name() -> String {
            //         #shadow_table_name.to_string()
            //     }
            // }
            // impl #name {
            //     //#shadow_t_func
            //
            //     #from_shadow_t_f
            // }
        }
    }
}