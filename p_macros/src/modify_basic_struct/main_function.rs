use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataStruct, LitStr};
use crate::additional_functions::construct_table::create_construct_table;
use crate::modify_basic_struct::create_shadow_table::create_shadow_table;
use crate::modify_basic_struct::get_shadow_table::get_shadow_table;
use crate::modify_basic_struct::update_basic_struct::update_fields;

pub fn create_macro(data: DataStruct, shadow_table_name_i: Ident, name: Ident, shadow_table_name: LitStr) -> proc_macro2::TokenStream {
    let construct_table_s = create_construct_table(&data);

    let updated_struct = update_fields(data, &name);

    let shadow_t_func = get_shadow_table(&construct_table_s, &shadow_table_name_i, &name);

    let shadow_table = create_shadow_table(&construct_table_s, &shadow_table_name_i);
    //connect
    quote!{
        pub mod #shadow_table_name_i{
            use Db_shit::*;

            #[derive(Debug)]
            #[crate::impl_table]
            #shadow_table

            #updated_struct

            impl Entity for #name
            {
                fn get_table_name() -> String {
                    #shadow_table_name.to_string()
                }
            }

            #shadow_t_func
        }
    }
}