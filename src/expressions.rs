use crate::{Column, RawColumn};
use crate::literals::Lit;
use crate::operators::Operator;

/// each expression should be divided with (expr)
enum Expr {
    Exprs(Vec<Expr>),
    Lit(Lit),
    Column(RawColumn),
    OperatorExpr(Operator),
}
enum ArithmeticExpr {
    Add(Expr)
}
enum BitwiseExpr {
    And(Column, Column),

}