use proc_macro2::Ident;
use quote::quote;
use syn::DataStruct;

pub fn create_struct(table: &DataStruct, table_name: &Ident, entity_ident: &Ident) -> proc_macro2::TokenStream {
    let table_fields = table.fields.iter().map(|field| {
        field
    });

    quote! {
        struct #table_name {
            db_connection: Connection,
            entities: Vec<#entity_ident>,
            #(#table_fields), *
        }
    }
}