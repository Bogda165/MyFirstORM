use my_macros::{AutoQueryable, From, Queryable};
use crate::create_a_name::{AutoQueryable, Queryable};
use crate::expressions::Expression;
use crate::literals::Number::*;
use crate::literals::Literal::*;


///Time literal
// #[derive(Debug)]
// pub struct Time {
//
// }
//
// ///Date literal
// #[derive(Debug)]
// pub struct Date {
//
// }


impl AutoQueryable for f32 {}
impl AutoQueryable for i32 {}
impl Queryable for f32 {
    fn convert_to_query(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Queryable for i32 {
    fn convert_to_query(&self) -> Option<String> {
        Some(self.to_string())
    }
}

/// Numbers literal change later to 64 instead of 32
#[derive(Debug, Queryable, Clone, AutoQueryable, From)]
#[path = "crate::literals"]
pub enum Number {
    Real(f32),
    Int(i32),
}


//Bool literal
#[derive(Debug, Clone, Queryable, AutoQueryable)]
#[path = "crate::literals"]
pub enum Bool {
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
    NULL,
    Bool(Bool),
    //CurrentTime(Time),
    //CurrentData(Date),
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

        let lit = Literal::StringLit("SomeString".to_string());
        assert_eq!(exclude_braces(lit.to_query()), "SomeString");
    }
}