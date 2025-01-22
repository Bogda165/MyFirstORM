use crate::{Column, RawColumn};
use crate::literals::Lit;
use crate::operators::Operator;

/// each expression should be divided with (expr)
///
/// The check will be performed when create for example some_expr.and(some_expr) check that expr is not Lit::String
enum Expr {
    Exprs(Vec<Expr>),
    Lit(Lit),
    Column(RawColumn),
    OperatorExpr(Operator),
}