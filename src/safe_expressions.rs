use std::marker::PhantomData;
use crate::column::{Allowed, Column, Table};
use crate::convertible::TheType;
use crate::create_a_name::{AutoQueryable, Queryable};
use crate::expressions::{Expression, RawTypes};
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

mod tests {
    use crate::create_a_name::Queryable;
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