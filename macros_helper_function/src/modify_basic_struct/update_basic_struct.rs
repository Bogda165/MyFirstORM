use proc_macro2::Ident;
use quote::quote;
use syn::{parse_quote, DataStruct, Fields};

pub fn update_fields(data: &mut DataStruct, name: &Ident) -> proc_macro2::TokenStream {
    crate::new_macros::table_def::impl_table((data, name), false, quote!{
        #[derive(Default, p_macros::OrmTable)]
        #[attrs_to_comments]
    })
}