use std::fmt::format;
use crate::column::Column;
use crate::queryable::{AutoQueryable, Queryable};
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
