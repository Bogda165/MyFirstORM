use proc_macro2::Ident;
use quote::quote;
use syn::DataStruct;
use crate::repo_struct::define_repo_struct::create_struct;
use crate::repo_struct::impl_repo_struct::impl_repo_struct;

pub fn init_repo_struct(repo: &DataStruct, repo_name: &Ident, entity_ident: &Ident, table_ident: &Ident) -> proc_macro2::TokenStream{
    let def_struct = create_struct(repo, repo_name, entity_ident);

    let impl_struct =  impl_repo_struct(entity_ident, repo_name, table_ident);

    quote!{
        use crate::users::*;
        #def_struct
        #impl_struct
    }
}