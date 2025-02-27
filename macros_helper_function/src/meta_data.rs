use quote::quote;
use std::collections::HashMap;
use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::Meta;

#[derive(Clone, Default)]
pub struct MetaData <'a>{
    pub attr_type: HashMap<&'a str, &'a str>,
}
impl<'a> MetaData<'a> {
    pub fn old_db() -> MetaData<'a> {
        let mut set = HashMap::new();
        set.insert("INTEGER",  "INTEGER");
        set.insert( "FLOAT",  "REAL");
        set.insert( "TEXT",  "TEXT");
        set.insert( "PK",  "PRIMARY KEY");
        set.insert( "AUTO_I",  "AUTOINCREMENT");
        set.insert("INTEGER_N", "INTEGER");
        set.insert("FLOAT_N", "FLOAT");
        set.insert("TEXT_N", "TEXT");
        set.insert( "CONNECT", "");
        MetaData {
            attr_type: set
        }
    }

    pub fn sqlite_rust_types() -> MetaData<'a> {
        MetaData {
            attr_type: HashMap::from([
                ("Int", "i32"),
                ("Real", "f32"),
            ]),
        }
    }
}

pub(crate) struct TempData<'a> {
    pub attr_type: HashMap<&'a str, TokenStream2>,
}

impl<'a> TempData<'a> {
    pub fn new() -> TempData<'a> {
        TempData { attr_type: HashMap::from([
            ("Int", quote!{i32}),
            ("Real", quote!{f32}),
            ("Text", quote!{String}),
        ])
        }

    }
}

