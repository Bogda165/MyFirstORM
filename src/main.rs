use std::fmt::format;
use crate::column::Column;
use crate::create_a_name::{AutoQueryable, Queryable};

mod expressions;
mod column;
mod literals;
mod operators;
mod create_a_name;
mod play_types;
mod safe_expressions;
mod convertible;
mod query;

#[derive(Debug, Clone, Default)]
struct RawColumn {
    table_name: String,
    name: String,
}

impl AutoQueryable for RawColumn{}

impl Queryable for RawColumn {
    fn convert_to_query(&self) -> Option<String> {
        Some(format!("{}.{}", self.table_name, if self.table_name.len() > 0 {self.name.clone()} else {"".to_string()}))
    }
}

// for check
pub trait Query {
    // fn select(self, columns: Vec<Column>) -> String {
    //     //creat a string query with given columns
    //     String::new()
    // }
}

fn main() {
    println!("Hello, world!");
}
