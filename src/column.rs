use my_macros::{AutoQueryable, Queryable};
use crate::create_a_name::Queryable;
use crate::expressions::Expression;
use crate::Column::*;

#[derive(Debug, AutoQueryable, Queryable)]
#[path = "crate::column"]
pub enum Column {
    Expr(Expression),
    All,
    AllFromTable(String),
}