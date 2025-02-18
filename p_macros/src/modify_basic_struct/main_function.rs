use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataStruct, LitStr};
use crate::additional_functions::construct_table::create_construct_table;
use crate::modify_basic_struct::create_shadow_table::create_shadow_table;
use crate::modify_basic_struct::from_shadow_table_f::from_shadow_table_f;
use crate::modify_basic_struct::get_shadow_table::get_shadow_table;
use crate::modify_basic_struct::update_basic_struct::update_fields;

pub fn create_macro(data: DataStruct, shadow_table_name_i: Ident, name: Ident, shadow_table_name: LitStr) -> proc_macro2::TokenStream {
    let construct_table_s = create_construct_table(&data);

    let updated_struct = update_fields(data, &name);

    //let shadow_t_func = get_shadow_table(&construct_table_s, &shadow_table_name_i, &name);

    //let shadow_table = create_shadow_table(&construct_table_s, &shadow_table_name_i);

    let from_shadow_t_f = from_shadow_table_f(&shadow_table_name_i, &name, &construct_table_s);

    let shadow_table_name_s = shadow_table_name_i.to_string();
    //connect
    quote!{
        pub mod #shadow_table_name_i{
            use Db_shit::*;

            //#[derive(Debug)]
            //#shadow_table

            //#[doc = #shadow_table_name_s]
            #updated_struct

            impl Entity for #name
            {
                fn get_table_name() -> String {
                    #shadow_table_name.to_string()
                }
            }
            impl #name {
                //#shadow_t_func

                #from_shadow_t_f
            }
        }
    }
}