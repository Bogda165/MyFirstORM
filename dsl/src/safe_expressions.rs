use std::marker::PhantomData;
use crate::column::{Allowed, Column, Table};
use crate::convertible::TheType;
use crate::queryable::{AutoQueryable, Queryable};
use crate::expressions::{Expression};
use crate::expressions::raw_types::RawTypes;
pub trait SafeExprTuple<Tables> {

}

impl<'a, T: TheType, U> SafeExprTuple<U> for (SafeExpr<T, U>, &'a str) {}

impl<'a, T: TheType, U> AutoQueryable for (SafeExpr<T, U>, &'a str) {}

impl<'a, T: TheType, U> Queryable for (SafeExpr<T, U>, &'a str)
where
    (SafeExpr<T, U>, &'a str): AutoQueryable
{
    fn convert_to_query(&self) -> Option<String> {
        Some(format!("{} as {}", self.0.to_query(), self.1))
    }
}

impl<T: TheType, U, Tuple: SafeExprTuple<U>> SafeExprTuple<U> for ((SafeExpr<T, U>, &str), Tuple) {}

pub struct SafeExpr<ExprType: TheType, AllowedTables> {
    pub tables: PhantomData<AllowedTables>,
    pub(crate) type_val: PhantomData<ExprType>,
    pub(crate) expr: Expression,
}
impl<T: TheType, U> AutoQueryable for SafeExpr<T, U> {}

impl<T: TheType, U> Queryable for SafeExpr<T, U> {
    fn convert_to_query(&self) -> Option<String> {
        Some(self.expr.to_query())
    }
}

impl<ExprType: TheType, AllowedTables> SafeExpr<ExprType, AllowedTables>
{
    pub fn new(expr: Expression) -> Self {
        SafeExpr{
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<ExprType>,
            expr,
        }
    }

    pub fn change_tables<NewAllowedTables>(self) -> SafeExpr<ExprType, NewAllowedTables> {
        SafeExpr {
            tables: PhantomData::<NewAllowedTables>,
            type_val: self.type_val,
            expr: self.expr,
        }
    }

    // list of functions that accept self and return SafeExpr<U>, where T can different than U

    pub fn to_string(self) -> SafeExpr<String, AllowedTables> {
        SafeExpr{
            type_val: PhantomData::<String>,
            tables: PhantomData::<AllowedTables>,
            expr: self.cast::<String>().expr,
        }
    }

    pub fn literal(val: ExprType) -> SafeExpr<ExprType, AllowedTables>
    where
        ExprType: Into<RawTypes>
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<ExprType>,
            expr: Expression::Raw(val.into()),
        }
    }

    pub fn column() -> SafeExpr<ExprType, AllowedTables>
    where
        ExprType: Column,
        AllowedTables: Allowed<<ExprType as Column>::Table>,
    {
        SafeExpr::new(Expression::Raw(ExprType::into(ExprType::default())))
    }
}

pub fn column<Type: TheType + Column, AllowedTables: Allowed<<Type as Column>::Table>>(column: Type) -> SafeExpr<Type, AllowedTables> {
    SafeExpr::<Type, AllowedTables>::column()
}

pub fn literal<ExprType: TheType, AllowedTables>(val: ExprType) -> SafeExpr<ExprType, AllowedTables>
where
    ExprType: Into<RawTypes>
{
    SafeExpr::literal(val)
}



pub mod safe_case {
    use crate::convertible::ConvertibleTo;
    use super::*;
    use crate::expressions::CaseExpr;
    use crate::literals::Bool;

    pub struct SafeCase<ThenT: TheType, AllowedTables> {
        tables: PhantomData<AllowedTables>,
        _marker: PhantomData<ThenT>,
        case_expr: CaseExpr,
    }

    impl<ThenT: Default + TheType, AllowedTables: Table> SafeCase<ThenT, AllowedTables> {
        pub fn when_do<T: TheType, U: TheType>(mut self, when: SafeExpr<T, AllowedTables>, then: SafeExpr<U, AllowedTables>) -> Self
        where
            T::Type: ConvertibleTo<Bool>,
            U::Type: ConvertibleTo<ThenT::Type>,
        {
            self.case_expr.case.push((when.expr, then.expr));
            self
        }

        pub fn end(self) -> SafeExpr<ThenT, AllowedTables>
        {
            SafeExpr {
                tables: PhantomData::<AllowedTables>,
                type_val: PhantomData::<ThenT>,
                expr: Expression::CaseExpr(Box::new(self.case_expr)),
            }
        }
    }


    impl<T: TheType, AllowedTables> SafeExpr<T, AllowedTables> {
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
}

mod tests {
    use crate::queryable::Queryable;
    use crate::expressions::Expression;
    use crate::literals::{Literal, Number};
    use crate::safe_expressions::SafeExpr;

    #[test]
    fn get_basic_type() {
        let literal: SafeExpr<_, ()> = SafeExpr::literal(Number::Int(10));
        let check = SafeExpr::<Literal, ()>::new(Expression::Raw(Literal::NumberLit(10.into()).into()).into());

        println!("{}", literal.expr.to_query());

        assert_eq!(literal.expr.to_query(), check.expr.to_query())
    }
}