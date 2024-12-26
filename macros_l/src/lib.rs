extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;
use proc_macro::Span;
use proc_macro2::extra::DelimSpan;
use quote::quote;
use syn::{parse, parse2, parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, ItemFn, ItemStruct, LitStr, Token, Type, TypeTuple};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;

fn get_variants_fn(input: DeriveInput) -> Vec<String> {
    match input.data {
        Data::Struct(data) => {
            data.fields.into_iter().map(|variant| {
                variant.ident.unwrap().to_string()
            }).collect::<Vec<String>>()
        }
        Data::Enum(data) => {
            data.variants.into_iter().map(|variant| {
                variant.ident.to_string()
            }).collect::<Vec<String>>()
        }
        Data::Union(_) => {unreachable!()}
    }
}

#[proc_macro_attribute]
pub fn fields_name(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = input.clone();
    let _input = input.clone();
    let name = input.ident;

    let values = get_variants_fn(_input);

    let values_q = quote! {
        [#(#values.to_string()), *]
    };

    TokenStream::from(quote!{
        #output
        impl #name {
            pub fn get_variants() -> Vec<String> {
                #values_q.to_vec()
            }
        }
    })
}

#[proc_macro_attribute]
pub fn get_types(_attr: TokenStream, input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let output = input.clone();
    let _input = input.clone();
    let name = input.ident;

    let values = match input.data {
        Data::Struct(ref data) => {
            unreachable!("Realise this feature later")
        }
        Data::Enum(ref data) => {
            data.variants.iter().map(|variant| {
                let data_wi_enum = &variant.fields;
                match data_wi_enum {
                    Fields::Named(_) => {
                        unreachable!("Realise this feature later")
                    }
                    Fields::Unnamed(unnamed_field) => {
                        let mut elements: Punctuated<Type, Token![,]> = Punctuated::new();

                        unnamed_field.unnamed.iter().for_each(|field|{
                            let _ty = field.ty.clone();
                            elements.push(_ty);
                        });
                        let mut tuple_type;
                        if elements.len() != 1 {
                            tuple_type = quote!{
                                (#elements)
                            };
                        }

                        tuple_type = quote!{
                            #elements
                        };

                        quote! {
                            parse2(quote!{#tuple_type}).unwrap()
                        }
                    }
                    Fields::Unit => {
                        unreachable!("Realise this feature later")
                    }
                }
            })
        }
        Data::Union(_) => {unreachable! ("Realise this feature later")}
    };

    let _values = values.clone();

    let names = get_variants_fn(_input);

    let for_map = _values.zip(names.into_iter()).map(|(typ, name)| {
        quote! {
            _return.insert(#name.to_string(), #typ);
        }
    });


    TokenStream::from(quote!{
        #output
        impl #name {
            pub fn get_types_v() -> Vec<Type> {
                vec![#(#values),*]
            }

            pub fn get_types_nt() -> HashMap<String, Type> {
                let mut _return = HashMap::new();
                #(#for_map)*
                _return
            }
        }
    })
}
