use std::panic;
use std::str::FromStr;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use syn::{parse_quote, AttrStyle, Attribute, Meta, MetaList};
use syn::MacroDelimiter::Paren;
use Db_shit::{Attributes, DbTypes};
use crate::meta_data::MetaData;

pub fn is_in_allowed_attrs(input: &Ident) -> Result<((proc_macro2::TokenStream)), (())> {
    let input_name = input.to_string();
    let attrs = Attributes::get_variants();
    let types = DbTypes::get_variants();

    if attrs.contains(&input_name) {
        Ok(quote! {
            Attributes
        })
    }else if types.contains(&input_name) {
        Ok(quote!{
            DbTypes
        })
    } else {
        return Err(())
    }

}

pub fn create_attr_with_type(input: &Attribute, field_name: Ident) -> Result<((proc_macro2::TokenStream)), (())> {
    let input_s = input.meta.path().get_ident().unwrap().to_string();
    let attrs = Attributes::get_variants();
    let types = DbTypes::get_variants();
    let input_ident = input.meta.path().get_ident().unwrap();

    if attrs.contains(&input_s) {
        Ok(match input.meta.require_list(){
            Ok(list) => {
                quote!{
                    Attributes::#list
                }
            }
            Err(_) => {
                quote!{
                    Attributes::#input_ident
                }
            }
        })
    }else if types.contains(&input_s) {
        Ok(quote!{
            DbTypes::#input_ident(self.#field_name.clone())
        })
    } else {
        return Err(())
    }
}

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
