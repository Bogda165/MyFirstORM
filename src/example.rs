use orm_traits::db::OrmDataBase;
use dsl::column::{Allowed, Table};
use dsl::query::the_query::Query;
use dsl::convertible::TheType;
use dsl::{from, from_tables, query_from};
use dsl::queryable::Queryable;
use dsl::safe_expressions::{column, literal};
use p_macros::{repo, attrs_to_comments, data_base};
use p_macros::table;
use rusqlite::{params, Error, Params, Row};
use orm_traits::attributes::*;

use crate::address::*;
use crate::users::*;
use p_macros::OrmTable;
use orm_traits::{OrmColumn, OrmTable};
use rusqlite::{Connection, OpenFlags};
use rusqlite::types::Value;
use std::any::Any;
use std::ops::{Deref, DerefMut};
use load_logic::code::*;
use orm_traits::relations::relation_types::{HaveRelationWith, OneToOne, RelationStruct};

use crate::example::table1::Table1;
use crate::example::table2::Table2;
#[derive(Default, Clone, Debug)]
#[table("table1")]
struct Table1 {
    #[column]
    #[sql_type(Int)]
    pub id: i32,
    #[column]
    #[sql_type(Text)]
    pub text: String,
    #[column]
    #[sql_type(Int)]
    pub table2_id: i32,
}


impl table1::Table1 {
    pub fn new(table2_id: i32) -> Self {
        let mut tmp: Table1 = Default::default();

        tmp.table2_id = table2_id;

        tmp
    }
}

#[derive(Clone, Default, Debug)]
#[table("table2")]
struct Table2 {
    #[column]
    #[sql_type(Int)]
    pub id_2: i32,
    #[column]
    #[sql_type(Int)]
    pub table1_id: i32,
}

impl table2::Table2 {
    pub fn new(table1_id: i32) -> Self {
        Self {
            id_2: 0,
            table1_id,
        }
    }

    pub fn get_table1(&self) -> Vec<Table1> {
        vec![Table1 {
            id: self.table1_id,
            text: "hui".to_string(),
            table2_id: self.id_2,
        }]
    }
}


// impl Table2 {
//     pub fn get_relation_with_table1<I: Iterator<Item = Self>>(iterator: I) -> RelationStruct<Self, Table1> {
//         let mut relation_struct: RelationStruct<Table2, Table1> = Default::default();
//         iterator.into_iter().map(|obj| {
//             RelationStruct::new(Some(vec![obj.clone()]), Some(obj.get_table1()))
//         }).for_each(|sub_struct| {
//             relation_struct.deref_mut().extend(sub_struct.into_iter());
//         });
//
//         relation_struct
//     }
// }
impl HaveRelationWith<table1::Table1, i32> for table2::Table2 {
    type RType = OneToOne;
    type SelfIdent = table2::table1_id;
}

impl HaveRelationWith<table2::Table2, i32> for table1::Table1 {
    type RType = OneToOne;
    type SelfIdent = table1::table2_id;
}

#[test]
fn some_test() {
    let tables1 = vec![table1::Table1::new(10), table1::Table1::new(2)];
    let tables2 =  vec![table2::Table2::new(2), table2::Table2::new(15)];

    let rel_struct: RelationStruct<Table1, Table2, _> = RelationStruct::new(Some(tables1), Some(tables2));

    println!("{:?}", rel_struct);
}