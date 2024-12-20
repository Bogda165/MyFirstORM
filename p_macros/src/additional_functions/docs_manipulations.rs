use quote::quote;
use syn::{Attribute, Meta};
use syn::Expr::{Lit};
use proc_macro::{TokenStream, TokenTree};
use syn::Lit::Str;
use crate::additional_functions::attributes_manipulations::parse_string_to_attr;

pub fn from_attribute_to_comment(attr: Attribute) -> proc_macro2::TokenStream {
    let name = attr.meta.path().get_ident().unwrap().to_string();
    let additional = match attr.meta {
        Meta::Path(path) => {
            "".to_string()
        }
        Meta::List(metaList) => {
            let mut token_stream = TokenStream::from(metaList.tokens);
            format!("({})", token_stream.into_iter().map(|tree| {
                match tree {
                    TokenTree::Group(_) => { unreachable!("write converting to a group") }
                    TokenTree::Ident(ident) => { ident.to_string()}
                    TokenTree::Punct(_) => {"".to_string()}
                    TokenTree::Literal(lit) => { lit.to_string() }
                }
            }).filter(|str| str.len() > 0).collect::<Vec<String>>().join(", "))
        }
        Meta::NameValue(_) => {
            unreachable!("Idk what to do if so")
        }
    };
    let _return = format!("{}{}", name, additional);
    quote! {
        #[doc = #_return]
        //I mean its strange
    }
}

pub fn from_doc_text_to_ident(doc: &Attribute) -> Result<Attribute, String> {
    if doc.meta.path().get_ident().unwrap().to_string() == "doc" {
        let meta_name_value = doc.meta.require_name_value().unwrap();
        if let Lit(value_in_doc) = &meta_name_value.value {
            if let Str(value_in_doc_string) = &value_in_doc.lit {
                let value = value_in_doc_string.value().to_string();
                let attr = parse_string_to_attr(value)?;
                return Ok(attr)
            }
            return Err("Not a String".to_string());
        }
        return Err("Not a Lit".to_string());
    }
    Err("Not a doc".to_string())
}
