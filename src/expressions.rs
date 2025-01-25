use crate::expressions::Expression::*;
use std::io::read_to_string;
use my_macros::{AutoQueryable};
use crate::{Column, RawColumn};
use crate::create_a_name::{AutoQueryable, Queryable};
use crate::literals::Literal;
use crate::operators::Operator;


/// each expression should be divided with (expr)
///
/// The check will be performed when create for example some_expr.and(some_expr) check that expr is not Lit::String
#[derive(Debug, AutoQueryable, Clone)]
#[path = "crate::expressions"]
pub enum Expression {
    Lit(Literal),
    Column(RawColumn),
    /// as operator must require an Expr inside to avoid non sized object use Box
    OperatorExpr(Box<Operator>),
    /// case expression must contain another Expr
    CaseExpr(Box<CaseExpr>),
}

impl Queryable for Expression{
    fn convert_to_query(&self) -> Option<String> {
        None
    }
}


///if base is non case must contain a row or binary operator
#[derive(Debug, Clone)]
struct CaseExpr {
    base_expr: Option<Expression>,
    case: Vec<(Expression, Expression)>,
    else_expr: Expression,
}

impl AutoQueryable for CaseExpr{}

impl Queryable for CaseExpr {
    fn convert_to_query(&self) -> Option<String> {
        let mut when_params:String = String::new();
        self.case.iter().for_each(|(expr1, expr2)| {
            when_params += &*format!("WHEN {} THEN {}\n", expr1.to_query(), expr2.to_query());
        });

        Some(format!{"CASE {base_expr} {when_recursion} ELSE {else_expr} END",
            base_expr = if let Some(expr) = &self.base_expr {expr.to_query()} else {"".to_string()},
            when_recursion = when_params,
            else_expr = &(self.else_expr).to_query(),
        })
    }
}

impl AutoQueryable for String {}

impl Queryable for String {
    fn convert_to_query(&self) -> Option<String> {
        Some(self.clone())
    }
}
