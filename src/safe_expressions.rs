use crate::expressions::{Expression, RawTypes};
pub struct SafeExpr<T> {
    pub(crate) type_val: T,
    pub(crate) expr: Expression,
}

impl<T> SafeExpr<T> {
    pub fn new(type_val: T, expr: Expression) -> Self {
        SafeExpr{
            type_val,
            expr,
        }
    }

    // list of functions that accept self and return SafeExpr<U>, where T can different than U

    pub fn to_string(self) -> SafeExpr<String> {
        SafeExpr{
            type_val: String::default(),
            expr: self.expr,
        }
    }

    pub fn basic(val: T) -> SafeExpr<T>
    where
        T: Into<RawTypes> + Clone
    {
        SafeExpr {
            type_val: val.clone(),
            expr: Expression::Raw(val.into()),
        }
    }
}

mod tests {
    use crate::create_a_name::Queryable;
    use crate::expressions::Expression;
    use crate::literals::{Literal, Number};
    use crate::safe_expressions::SafeExpr;

    #[test]
    fn get_basic_type() {
        let basic = SafeExpr::basic(Number::Int(10));
        let check = SafeExpr::new(Literal::NumberLit(10.into()), Expression::Raw(Literal::NumberLit(10.into()).into()).into());

        println!("{}", basic.expr.to_query());

        assert_eq!(basic.expr.to_query(), check.expr.to_query())
    }
}