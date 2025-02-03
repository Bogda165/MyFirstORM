use crate::expressions::Expression::*;
use std::io::read_to_string;
use std::iter::Filter;
use std::marker::PhantomData;
use my_macros::{AutoQueryable, From, Queryable};
use crate::{Column, RawColumn};
use crate::column::Table;
use crate::convertible::ConvertibleTo;
use crate::create_a_name::{AutoQueryable, Queryable};
use crate::literals::{Bool, Literal, Number};
use crate::operators::Operator;
use crate::safe_expressions::SafeExpr;

#[derive(Debug, AutoQueryable, Clone, Queryable, From)]
#[path = "crate::expressions"]
pub enum RawTypes {
    Lit(Literal),
    Column(RawColumn),
}

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
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Raw(RawTypes::Lit(value))
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

struct SafeCase<ThenT, AllowedTables> {
    tables: PhantomData<AllowedTables>,
    _marker: PhantomData<ThenT>,
    case_expr: CaseExpr,
}

impl<ThenT: Default, AllowedTables: Table> SafeCase<ThenT, AllowedTables> {
    pub fn when_do<T, U>(mut self, when: SafeExpr<T, AllowedTables>, then: SafeExpr<U, AllowedTables>) -> Self
    where
        T: ConvertibleTo<Bool>,
        U: ConvertibleTo<ThenT>,
    {
        self.case_expr.case.push((when.expr, then.expr));
        self
    }

    pub fn end(self) -> SafeExpr<ThenT, AllowedTables>
    where ThenT: Default
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<ThenT>,
            expr: Expression::CaseExpr(Box::new(self.case_expr)),
        }
    }
}


impl<T, AllowedTables> SafeExpr<T, AllowedTables> {
    pub fn case_else(else_expr: SafeExpr<T, AllowedTables>) -> SafeCase<T, AllowedTables>
    {
        SafeCase::<T, AllowedTables> {
            tables: PhantomData::<AllowedTables>,
            _marker: PhantomData,
            case_expr: CaseExpr {
                base_expr: None,
                case: vec![],
                else_expr: else_expr.expr,
            }
        }
    }
}

impl AutoQueryable for String {}

impl Queryable for String {
    fn convert_to_query(&self) -> Option<String> {
        Some(format!("\"{}\"", self.clone()))
    }
}


mod tests {
    use crate::create_a_name::Queryable;
    use crate::expressions::RawTypes;
    use crate::safe_expressions::SafeExpr;

    // #[test]
    // fn case_expr() {
    //     let some_val: SafeExpr<_, ()> =
    //         SafeExpr::case_else(
    //             SafeExpr::<_, ()>::basic("hello".to_string())
    //         )
    //         .when_do(
    //             SafeExpr::basic(10)
    //                 .more(SafeExpr::basic(14)),
    //             SafeExpr::basic("its more man".to_string()))
    //         .when_do(
    //             SafeExpr::basic(10)
    //                 .to_string()
    //                 .like("%10", None),
    //             SafeExpr::basic("Its correct string".to_string())
    //         )
    //         .end();
    //
    //     println!("{}", some_val.expr.to_query());
    // }
}