extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::quote;
use syn::{parse, parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, ItemFn, ItemStruct, LitStr};

#[proc_macro_attribute]
pub fn fields_name(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = input.clone();
    let name = input.ident;


    let data = match input.data {
        Data::Struct(_) => {unreachable!()}
        Data::Enum(data) => {
            data
        }
        Data::Union(_) => {unreachable!()}
    };

    let fields = data.variants.into_iter().map(|variant| {
        let name = variant.ident.to_string();
        name
    });

    let values = quote!{
        [#(#fields.to_string()), *]
    };

    TokenStream::from(quote!{
        #output
        impl #name {
            pub fn get_variants() -> Vec<String> {
                #values.to_vec()
            }
        }
    })
}