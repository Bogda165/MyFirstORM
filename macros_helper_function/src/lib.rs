use syn::{Path, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub mod meta_data;
pub mod impl_for_shadow_table;
pub mod modify_basic_struct;
pub mod repo_struct;
pub mod additional_functions;
pub mod new_macros;
pub mod derive_orm_traits;
pub mod load_funcs;
mod relations;

pub mod custom_parser{
    use proc_macro2::Ident;
    use syn::{Error, LitStr};
    use super::*;
    pub struct CommaPath {
        value: Punctuated<Path, Token![,]>,
    }

    pub enum ParsingState {
        Comma,
        Field
    }

    impl ParsingState {
        fn change(&mut self) {
            match self {
                ParsingState::Comma => { *self = ParsingState::Field }
                ParsingState::Field => { *self = ParsingState::Comma }
            }
        }
    }

    impl Parse for CommaPath {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut res = CommaPath {value: Punctuated::default()};
            let mut state = ParsingState::Field;
            while(!input.is_empty()) {
                match state {
                    ParsingState::Comma => { input.parse::<Token![,]>()?; }
                    ParsingState::Field => { res.value.push(input.parse::<Path>()?); }
                };
                state.change();
            }

            Ok(res)
        }
    }

    impl Into<Vec<Path>> for CommaPath {
        fn into(self) -> Vec<Path> {
            self.value.into_iter().map(|element| {element}).collect()
        }
    }


    pub struct KeyValue {
        pub key: Ident,
        pub value: LitStr,
    }

    pub struct KeyValueList {
        pairs: Punctuated<KeyValue, Token![,]>,
    }

    impl Parse for KeyValueList {
        fn parse(input: ParseStream) -> Result<Self, Error> {
            let pairs = Punctuated::parse_terminated(input)?;
            Ok(KeyValueList { pairs })
        }
    }

    impl IntoIterator for KeyValueList {
        type Item = KeyValue;
        type IntoIter = <syn::punctuated::Punctuated<KeyValue, syn::token::Comma> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.pairs.into_iter()
        }
    }

    impl Parse for KeyValue {
        fn parse(input: ParseStream) -> Result<Self, syn::Error> {
            let key: Ident = input.parse().unwrap();
            eprintln!("{}", key.to_string());

            eprintln!("{}", input);
            input.parse::<Token![=]>().unwrap();
            let value: LitStr = input.parse().unwrap();

            eprintln!("{}", value.value());
            Ok(KeyValue { key, value })
        }
    }

}

pub mod junk {
    pub struct FroConnect {
        pub(crate) path: String,
        pub(crate) table_name: String,
        pub(crate) field_name: String,
    }

    impl FroConnect {
        pub(crate) fn default() -> Self {
            FroConnect {
                path: "".to_string(),
                table_name: "".to_string(),
                field_name: "".to_string(),
            }
        }
    }
}