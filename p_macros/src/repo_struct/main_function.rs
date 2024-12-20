use proc_macro2::Ident;
use quote::quote;
use syn::DataStruct;
use crate::repo_struct::define_repo_struct::create_struct;
use crate::repo_struct::impl_repo_struct::impl_repo_struct;

pub fn init_repo_struct(table: &DataStruct, table_name: &Ident, entity_ident: &Ident) -> proc_macro2::TokenStream{
    let def_struct = create_struct(table, table_name, entity_ident);

    let impl_struct =  impl_repo_struct(entity_ident, table_name);

    quote!{
        use crate::users::*;
        #def_struct
        #impl_struct
    }
}