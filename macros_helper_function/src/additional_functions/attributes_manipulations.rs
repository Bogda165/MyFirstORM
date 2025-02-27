use std::panic;
use std::str::FromStr;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use syn::{parse_quote, AttrStyle, Attribute, Meta, MetaList};
use syn::MacroDelimiter::Paren;
use crate::meta_data::MetaData;


pub fn to_string<'a, 'b>(attr: &'a Ident, meta_data: MetaData<'b>) -> &'b str {
    let attr_name = &*attr.to_string();

    match meta_data.attr_type.get(attr_name) {
        None => { std::panic!("No type or attribute in the list, {}", attr_name)}
        Some(val) => {
            val
        }
    }
}


pub fn parse_string_to_attr(attr_str: String) -> Result<Attribute, String> {
    // get a type of Meta

    let meta = if let Some(index) = attr_str.find('(') {
        //list
        let ident_str = &(&*attr_str)[..index];
        let len = attr_str.len();
        let ident = Ident::new(ident_str, Span::call_site());
        // get between brackets
        let tokens_iter = (&*attr_str)[(index + 1)..len - 1].split(", ").map(|str| {
            match panic::catch_unwind(|| {Ident::new(str, Span::call_site())}) {
                Ok(ident) => {
                    quote!{
                        #ident
                    }
                }
                Err(_) => {
                    let lit = Literal::from_str(str).unwrap();

                    quote! {
                        #lit
                    }
                }
            }
        });
        let tokens = quote! {
            #(#tokens_iter),*
        };

        eprintln!("{:?}", tokens.to_string());

        let hui = MetaList {
            path: parse_quote!(#ident),
            delimiter: Paren(syn::token::Paren::default()),
            tokens,
        };

        Meta::List(hui)
    }else if let Some(index) = attr_str.find("=") {
        //nameVal
        unreachable!("Not realized for name val")
    }else {
        //path
        let ident_str = &*attr_str;

        let ident = Ident::new(ident_str, Span::call_site());
        Meta::Path(parse_quote!(#ident))
    };



    Ok(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta,
    })
}
