use std::collections::HashMap;
use proc_macro2::Ident;
use syn::{Attribute, DataStruct};
use crate::additional_functions::docs_manipulations::from_doc_text_to_ident;

pub fn create_construct_table(structure: &DataStruct) -> HashMap<Ident, Vec<Attribute>>{
    let mut construct_table_s = HashMap::<Ident, Vec<Attribute>>::new();
    for field in structure.fields.iter() {
        construct_table_s.insert(field.clone().ident.unwrap(), field.clone().attrs);
    }
    construct_table_s
}

pub fn create_construct_table_from_doc(structure: &DataStruct) -> HashMap<Ident, Vec<Attribute>>{
    let mut construct_table_s = HashMap::<Ident, Vec<Attribute>>::new();
    for field in structure.fields.iter() {
        //extract a value from a string
        let _attrs: Vec<Attribute> = field.attrs.iter().map(|attribute| {
            from_doc_text_to_ident(attribute).unwrap()
        }).collect();


        construct_table_s.insert(field.clone().ident.unwrap(), _attrs);
    }
    construct_table_s
}