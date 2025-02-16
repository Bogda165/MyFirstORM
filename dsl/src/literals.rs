use my_macros::{AutoQueryable, From, Queryable};
use crate::queryable::{AutoQueryable, Queryable};
use crate::expressions::Expression;
use crate::literals::Number::*;
use crate::literals::Literal::*;

#[derive(Debug, Clone, Default)]
pub struct Null {

}

impl AutoQueryable for Null {}

impl Queryable for Null {
    fn convert_to_query(&self) -> Option<String> {
       Some("NULL".to_string())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Time {

}

impl AutoQueryable for Time {}
impl Queryable for Time {
    fn convert_to_query(&self) -> Option<String> {
        Some("time".to_string())
    }
}

///Date literal
#[derive(Debug, Clone, Default)]
pub struct Date {

}

impl AutoQueryable for Date {}

impl Queryable for Date {
    fn convert_to_query(&self) -> Option<String> {
        Some("Date".to_string())
    }
}

/// Numbers literal change later to 64 instead of 32
#[derive(Debug, Queryable, Clone, AutoQueryable, From)]
#[path = "crate::literals"]
pub enum Number {
    Real(f32),
    Int(i32),
}

impl Default for Number {
    fn default() -> Self {
        Number::Int(0)
    }
}


//Bool literal
#[derive(Default, Debug, Clone, Queryable, AutoQueryable)]
#[path = "crate::literals"]
pub enum Bool {
    #[default]
    True,
    False
}


/// Literals
#[derive(Debug, Clone, Queryable, AutoQueryable, From)]
#[path = "crate::literals"]
pub enum Literal {
    NumberLit(Number),
    StringLit(String),
    BlobLit,
    NULL(Null),
    Bool(Bool),
    CurrentTime(Time),
    CurrentData(Date),
}

impl Default for Literal {
    fn default() -> Self {
        Null::default().into()
    }
}

mod tests {
    use crate::literals::{Bool, Literal, Number};
    use crate::Queryable;

    fn exclude_braces(mut query: String) -> String {
        query.replace("(", "").replace(")", "")
    }

    #[test]
    fn test1() {
        let lit = Literal::NumberLit(10.into());
        assert_eq!(exclude_braces(lit.to_query()), "10");

        let lit = Literal::NumberLit(Number::Real(10.24));
        assert_eq!(exclude_braces(lit.to_query()), "10.24");

        let lit = Literal::Bool(Bool::True);
        assert_eq!(exclude_braces(lit.to_query()), "True");

        let lit: Literal = Literal::StringLit("SomeString".into());
        assert_eq!(exclude_braces(lit.to_query()), "\"SomeString\"");
    }
}