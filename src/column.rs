use my_macros::Queryable;
use crate::create_a_name::Queryable;
use crate::expressions::Expression;
use crate::Column::*;

#[derive(Debug, Queryable)]
#[path = "crate::column"]
pub enum Column {
    Expr(Expression),
    All,
    AllFromTable(String),
}