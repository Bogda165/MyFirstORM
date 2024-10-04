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


    let values = match input.data {
        Data::Struct(data) => {
            let fields = data.fields.into_iter().map(|variant| {
                variant.ident.unwrap().to_string()
            });
            quote!{
                [#(#fields.to_string()), *]
            }
        }
        Data::Enum(data) => {
            let fields = data.variants.into_iter().map(|variant| {
                variant.ident.to_string()
            });
            quote!{
                [#(#fields.to_string()), *]
            }
        }
        Data::Union(_) => {unreachable!()}
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

