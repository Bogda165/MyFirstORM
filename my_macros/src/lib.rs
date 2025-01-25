use proc_macro::TokenStream;
use std::env::var;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, parse_str, Attribute, Data, DeriveInput, Field, Fields, Path, Token};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;

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


#[proc_macro_derive(Queryable, attributes(divide))]
pub fn my_custom_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

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
                        #path::#variant_name => {#vns.to_string()}
                    }
                }
            })
        }
        Data::Struct(_) => {unreachable!("No basic realisation")}
        Data::Union(_) => {unreachable!()}
    };


    let _return = quote! {
        impl Queryable for #enum_name {
            fn to_query_auto(&self) -> Option<String> {
                match self {
                    #(#inside_match),*
                }
            }
        }
    };

    TokenStream::from(_return)
}