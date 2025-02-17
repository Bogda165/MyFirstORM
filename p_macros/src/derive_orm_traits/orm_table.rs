use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Path, PathSegment, Token, Type};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;

pub fn orm_table_derive_f(table_name: Ident, table: DataStruct) -> TokenStream2{


    quote! {

    }
}