

mod meta_data;
mod impl_for_shadow_table;
mod modify_basic_struct;
mod repo_struct;
mod additional_functions;
mod new_macros;
mod derive_orm_traits;

extern crate proc_macro;
use proc_macro::{TokenStream};
use std::panic;
use std::str::FromStr;
use std::vec::IntoIter;
use proc_macro2::{Ident, Literal, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{parenthesized, parse, parse_macro_input, parse_quote, token, AttrStyle, Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, Item, ItemFn, ItemStruct, LitStr, MacroDelimiter, Meta, MetaList, Token, Type};
use syn::__private::TokenStream2;
use syn::Expr::Lit;
use syn::Lit::Str;
use syn::MacroDelimiter::Paren;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::token::{Struct, Token};
use Db_shit::*;
use crate::additional_functions::construct_table::create_construct_table_from_doc;
use crate::additional_functions::docs_manipulations::from_attribute_to_comment;
use crate::additional_functions::functions::{iter_through_attrs, orm_table_derive_f};
use crate::impl_for_shadow_table::main_function::generate_function;
use crate::meta_data::MetaData;
use crate::modify_basic_struct::main_function::create_macro;
use crate::repo_struct::main_function::init_repo_struct;

macro_rules! my_to_vec {
    ($vec:expr, Vec<$vec_type:ty>, $left:literal, $right:literal) => {
        {
            let _vec: [$vec_type; $right - $left] = $vec[..$right - $left].try_into().unwrap();
            _vec
        }
    }
}


#[proc_macro_derive(MyTrait2)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();

    impl_macro(&input)
}

fn find_by_name(path: &str, name: String) -> Result<ItemStruct, ()> {
    //convert to syn::File
    let syn_file = match syn::parse_file(path) {
        Ok(file) => file,
        Err(_) => {panic! ("Couldn't open a file")}
    };

    for item in syn_file.items {
        match item {
            Item::Struct(stru) => {
                if stru.ident.to_string() == name {
                    return Ok(stru.clone());
                }
            }
            _ => {}
        }
    }

    Err(())
}

fn impl_macro(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let _res = quote! {
        impl MyTrait2 for #name {
            fn print_hello_with_macro(&self) {
                println!("Hi my name is: {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(_res)
}

#[proc_macro_attribute]
pub fn log_exec_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;

    TokenStream::from(quote! {
        use std::time;
        fn #fn_name() {
            let start = time::Instant::now();
            (|| #block)();
            println!("Execition time: {:?} ms", start.elapsed().as_millis());
        }
    })
}


struct FroConnect {
    path: String,
    table_name: String,
    field_name: String,
}

impl FroConnect {
    fn default() -> Self {
        FroConnect {
            path: "".to_string(),
            table_name: "".to_string(),
            field_name: "".to_string(),
        }
    }
}



#[proc_macro_attribute]
pub fn impl_table(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //check for struct
    let input = parse_macro_input!(input as DeriveInput);
    let table_name_ident = &input.ident;
    let table = match &input.data {
        Data::Struct(table) => {table}
        _ => {
            std::panic!("The table must be represented as a struct")
        }
    };

    let construct_table_s = create_construct_table_from_doc(&table);
    eprintln!("Construct table create from comments");

    generate_function(&input, construct_table_s, table_name_ident)
}
#[proc_macro_attribute]
pub fn table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!("Start");

    let shadow_table_name = parse_macro_input!(_attr as LitStr);
    let shadow_table_name_i = Ident::new(&shadow_table_name.value(), shadow_table_name.span());
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.clone().ident;



    let macro_res = create_macro(input, shadow_table_name_i, name, shadow_table_name);

    TokenStream::from(macro_res)
}

/*
Common Attributes:

1.	PRIMARY KEY: Uniquely identifies a row.
2.	AUTOINCREMENT: Automatically increments integer values.
3.	NOT NULL: Ensures no null values.
4.	UNIQUE: Enforces uniqueness.
5.	CHECK: Applies custom constraints.
6.	DEFAULT: Specifies default values.
7.	COLLATE: Defines string collation rules.
8.	FOREIGN KEY: Establishes relationships between tables.
9.	ON CONFLICT: Specifies conflict-handling behavior.

*/

struct KeyValue {
    key: Ident,
    value: LitStr,
}

struct KeyValueList {
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

#[proc_macro_attribute]
pub fn repo(_attr: TokenStream, input: TokenStream) -> TokenStream {

    eprintln!("{_attr}");
    let attrs = parse_macro_input!(_attr as KeyValueList);

    let mut table_name = LitStr::new("", Span::call_site());
    let mut entity_name = LitStr::new("", Span::call_site());

    eprintln!("hui");

    //TODO refactor

    for val in attrs {
        if val.key == "entity" {
            entity_name = val.value;
        }
        else if val.key == "table" {
            table_name = val.value;
        }
    }

    eprintln!("{:?} {:?}", table_name.value(), entity_name.value());



    let entity_ident = Ident::new(&*entity_name.value(), Span::call_site());
    let table_ident = Ident::new(&*table_name.value(), Span::call_site());
    let repo = parse_macro_input!(input as DeriveInput);

    let repo_name = repo.ident;
    let repo = match repo.data {
        Data::Struct(table) => {table}
        Data::Enum(_) => {
            panic!("It should be a struct")
        }
        Data::Union(_) => {
            panic!("It should be a struct")
        }
    };

    TokenStream::from(init_repo_struct(&repo, &repo_name, &entity_ident, &table_ident))
}

#[proc_macro]
// This macro must be used in the root of the project to generate multiple tables types
pub fn from(input: TokenStream) -> TokenStream {

    let types = parse_macro_input!(input with Punctuated::<Type, Token![,]>::parse_terminated);

    TokenStream::from(crate::new_macros::table_def::impl_from(types))
}


#[proc_macro_attribute]
pub fn attrs_to_comments(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut table = parse_macro_input!(input as DeriveInput);

    attrs_to_comments_f(&mut table);

    TokenStream::from(quote!{
        #table
    })
}

fn attrs_to_comments_f(table: &mut DeriveInput){
    let new_fields = match &mut table.data {
        Data::Struct(_table) => {
            _table.fields.iter_mut().for_each(|field| {
                field.attrs.iter_mut().for_each(|attr| {
                    let new_attr = from_attribute_to_comment(attr.clone());
                    let parser = Attribute::parse_outer;
                    let new_attr_attr = parser.parse2(new_attr).unwrap_or_else(|_| vec![]);
                    *attr = new_attr_attr[0].clone()
                });
            })
        }
        Data::Enum(_) => {unreachable!()}
        Data::Union(_) => {unreachable!()}
    };
}

fn get_tuple_from_table(mut table: DataStruct, _attrs_name: &str) -> TokenStream2{
    let types = &mut table.fields.iter_mut().filter_map(|field| {
        let _res = iter_through_attrs(field, false, |field, attr_name, _| {
            if attr_name == _attrs_name  {
                let field_name = field.clone().ident.unwrap();
                Some(quote!{<#field_name as TheType>::Type})
            }else {None}
        });

        if _res.len() == 0 {
            None
        }else {
            Some(_res[0].clone())
        }
    });

    quote!{
        (#(#types), *)
    }
}

#[proc_macro_derive(OrmTable)]
pub fn orm_table_derive(input: TokenStream) -> TokenStream {
    let mut _table = parse_macro_input!(input as DeriveInput);

    let _final = orm_table_derive_f(_table);

    TokenStream::from(_final)
}


