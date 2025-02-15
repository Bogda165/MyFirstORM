use crate::expressions::Expression::*;
use my_macros::{AutoQueryable, From, Queryable};
use crate::column::RawColumn;
use crate::column::Table;
use crate::convertible::{ConvertibleTo, TheType};
use crate::expressions::raw_types::RawTypes;
use crate::queryable::{AutoQueryable, Queryable};
use crate::literals::{Literal};
use crate::operators::Operator;


pub mod raw_types {
    use my_macros::{AutoQueryable, From, Queryable};
    use crate::literals::Literal;
    use crate::queryable::{AutoQueryable, Queryable};
    use crate::column::RawColumn;

    #[derive(Debug, AutoQueryable, Clone, Queryable, From)]
    #[path = "crate::expressions"]
    pub enum RawTypes {
        Lit(Literal),
        Column(RawColumn),
    }

    mod impls_for_raw_types {
        use crate::literals::{Bool, Number};
        use super::*;
        impl From<Number> for RawTypes {
            fn from(value: Number) -> Self {
                RawTypes::Lit(Literal::NumberLit(value))
            }
        }

        impl From<String> for RawTypes {
            fn from(value: String) -> Self {
                RawTypes::Lit(Literal::StringLit(value))
            }
        }

        impl From<Bool> for RawTypes {
            fn from(value: Bool) -> Self {
                RawTypes::Lit(Literal::Bool(value))
            }
        }

        impl From<i32> for RawTypes {
            fn from(value: i32) -> Self {
                RawTypes::Lit(Literal::NumberLit(Number::Int(value)))
            }
        }

        impl From<f32> for RawTypes {
            fn from(value: f32) -> Self {
                RawTypes::Lit(Number::Real(value).into())
            }
        }
    }
}

/// each expression should be divided with (expr)
///
/// The check will be performed when create for example some_expr.and(some_expr) check that expr is not Lit::String
#[derive(Debug, AutoQueryable, Clone, From, Queryable)]
#[path = "crate::expressions"]
pub enum Expression {
    Raw(RawTypes),
    /// as operator must require an Expr inside to avoid non sized object use Box
    OperatorExpr(Box<Operator>),
    /// case expression must contain another Expr
    CaseExpr(Box<CaseExpr>),
    /// Just empty expression
    Empty,
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Empty
    }
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Raw(RawTypes::Lit(value))
    }
}


///if base is non case must contain a row or binary operator
#[derive(Debug, Clone)]
pub struct CaseExpr {
    pub base_expr: Option<Expression>,
    pub case: Vec<(Expression, Expression)>,
    pub else_expr: Expression,
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

mod tests {
    use crate::queryable::Queryable;
    use crate::expressions::RawTypes;
    use crate::safe_expressions::SafeExpr;

    #[test]
    fn case_expr() {
        let some_val: SafeExpr<_, ()> =
            SafeExpr::case_else(
                SafeExpr::<_, ()>::literal("hello".to_string())
            )
            .when_do(
                SafeExpr::literal(10)
                    .more(SafeExpr::literal(14)),
                SafeExpr::literal("its more man".to_string()))
            .when_do(
                SafeExpr::literal(10)
                    .to_string()
                    .like("%10", None),
                SafeExpr::literal("Its correct string".to_string())
            )
            .end();

        println!("{}", some_val.expr.to_query());
    }
}