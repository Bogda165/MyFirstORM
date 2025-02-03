mod type_play;
mod table_def;

use proc_macro::TokenStream;
use std::env::var;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, parse_str, Attribute, Data, DeriveInput, Expr, Field, Fields, Lit, Path, Token, Type};
use syn::__private::TokenStream2;
use syn::Member::Unnamed;
use syn::punctuated::Punctuated;
use crate::table_def::{impl_from, impl_table};

fn get_module_path(attrs: &Vec<Attribute>, enum_name: String) -> Path {
    let module_path = match attrs.iter().find(|attr| attr.path.is_ident("path")) {
        Some(attr) => {
            //eprintln!("Hello I am there");
            attr.tokens.to_string()
        },
        None => {panic!("No path provided possible error"); String::new()}
    };

    let module_path = &module_path[3..module_path.len()-1];
    let module_path = format!("{}::{}", module_path, enum_name);

    parse_str::<Path>(&*module_path).unwrap()
}

fn get_divide_operators(attrs: &Vec<Attribute>) -> Vec<String> {
    match attrs.iter().find(|attr| attr.path.is_ident("divide")) {
        None => {
            vec![",".to_string()]
        }
        Some(attr) => {
            let operators_string = attr.tokens.to_string();
            let operators_string = &operators_string[2..operators_string.len() - 2];
            operators_string.split(",").map(|str| format!(" {str} ")).collect()
        },
    }
}

fn create_format(size: usize, divide: Vec<String>) -> String {
    (1..size)
        .zip(divide.into_iter().cycle())
        .map(
            |(_, element)| {
                format!{"{} ", element}
            }
        )
        .collect::<Vec<String>>().join("")
    + "{}"
}


#[proc_macro_derive(AutoQueryable, attributes(divide))]
pub fn my_custom_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;
    // get geneircs
    let generics = &input.generics;

    let where_clause = if let Some(where_clause) = generics.clone().where_clause {
        quote!{#where_clause}
    }else {
        quote!{}
    };

    let path = get_module_path(&input.attrs, enum_name.to_string());

    let divide_operator = get_divide_operators(&input.attrs);

    let mut inside_match =  match input.data {
        Data::Enum(_enum) => {
            _enum.variants.clone().into_iter().zip(divide_operator.into_iter().cycle()).map(|(variant, divide)| {
                let variant_name = &variant.ident;
                let iter_through_fields = |fields: Punctuated<Field, Token![,]>| {
                    let size = fields.len();
                    fields.into_iter().zip(1..=size).map(|field| {

                        let field_name = match &field.0.ident {
                            None => {
                                let i =
                                Ident::new(&*format!("field{}", field.1), Span::call_site());
                                //eprintln!("new fields name: {}", i.to_string());
                                i
                            }
                            Some(name) => {name.clone()}
                        };
                        //eprintln!("hui {}", &field_name.to_string());
                        quote! {
                            #field_name
                        }
                    })
                };
                let format = format!("({})", vec!["{}"; (&variant.fields).len()].join(&*divide));

                //eprintln!("Format{}", format);
                // eprintln!("amount of fields: {}", variant.fields.len());

                let fields = match variant.fields.clone() {
                    Fields::Named(fields) => {
                        //eprintln!("named field of the size {}", fields.named.len());
                        iter_through_fields(fields.named).collect::<Vec<TokenStream2>>()
                    }
                    Fields::Unnamed(fields) => {
                        //eprintln!("unnamed field of the size {}", fields.unnamed.len());
                        iter_through_fields(fields.unnamed).collect::<Vec<TokenStream2>>()
                    }
                    Fields::Unit => {
                        vec![]
                    }
                };
                if fields.len() > 0 {
                    let fields_cloned = fields.clone();

                    quote! {
                        #path::#variant_name(#(#fields),*) => {
                            Some(format!{
                                #format,
                                #(#fields_cloned.to_query(),)
                                *
                            })
                        }
                    }
                }else {
                    let vns = variant_name.to_string();
                    quote!{
                        #[doc = "HUI"]
                        #path::#variant_name => {Some(#vns.to_string())}
                    }
                }
            })
        }
        Data::Struct(_) => {unreachable!("No basic realisation")}
        Data::Union(_) => {unreachable!()}
    };


    let _return = quote! {

        impl #generics crate::create_a_name::AutoQueryable for #enum_name #generics
        #where_clause
        {
            fn to_query_auto(&self) -> Option<String> {
                match self {
                    #(#inside_match),*
                }
            }
        }
    };

    TokenStream::from(_return)
}

#[proc_macro_derive(Queryable)]
pub fn dervive_none_f(input: TokenStream) -> TokenStream {
    let input =  parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let where_clause = if let Some(where_clause) = generics.clone().where_clause {
        quote!{#where_clause}
    }else {
        quote!{}
    };

    TokenStream::from(
        quote!{
            impl #generics Queryable for #name #generics
            #where_clause
            {
                fn convert_to_query(&self) -> Option<String> {
                    None
                }
            }
        }
    )
}

#[proc_macro_derive(From)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    if let Data::Enum(_enum) = &input.data {
        let functions = _enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;

            let (variant_type, inside_value) = if let Fields::Unnamed(vf) = &variant.fields{
                let vf = vf.clone();
                let variant_type = quote!{#vf};

                let inside_value = if vf.unnamed.len() > 1 {
                    let inside = 0..vf.unnamed.len();

                    quote!{
                        #(value.#inside), *
                    }
                }else {
                    quote!{value}
                };

                (variant_type, inside_value)
            }else {
                return quote!{}
            };


            quote!{
                impl From<#variant_type> for #name {
                    fn from(value: #variant_type) -> Self {
                        Self::#variant_name(#inside_value)
                    }
                }
            }
        });

        TokenStream::from(quote! {
            #(#functions)*
        })

    }else {
        panic!("no impl for this type 0f data")
    }
}


#[proc_macro]
pub fn get_literal_type(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as Expr);

    // Match the expression and extract the type of the literal
    let ty = match &input {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Int(_) => "i32 or u32", // Can be adjusted based on the literal
            Lit::Float(_) => "f64",
            Lit::Str(_) => "String or &str", // For string literals, we'll treat them as String or &str
            Lit::Bool(_) => "bool",
            _ => "Unknown  Literal",
        },
        _ => "Not a Literal",
    };

    // Generate the code that prints the type of the literal
    let expanded = quote! {
        {
            println!("Literal Type: {}", #ty);
        }
    };

    // Return the generated code
    TokenStream::from(expanded)
}

#[proc_macro]
// This macro must be used in the root of the project to generate multiple tables types
pub fn from(input: TokenStream) -> TokenStream {

    let types = parse_macro_input!(input with Punctuated::<Type, Token![,]>::parse_terminated);

    TokenStream::from(impl_from(types))
}

#[proc_macro_attribute]
pub fn table(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut table = parse_macro_input!(input as DeriveInput);

    TokenStream::from(impl_table(table))
}