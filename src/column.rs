use crate::Expr;

enum Column {
    Expr(Expr),
    ALL,
}