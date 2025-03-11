use crate::convertible::TheType;
use crate::queryable::{AutoQueryable, Queryable};
use crate::expressions::raw_types::RawTypes;

/// trait that should implement every table, impl with #[table] macro
pub trait Table {
    fn get_name() -> String;
}

impl Table for () {
    fn get_name() -> String {
        String::default()
    }
}

/// Trait of a column in the table, impl using #[table] macro, and attribute #[column]
pub trait Column: Default + TheType + Into<RawTypes>{
    /// Type of the columns table
    type Table;

    const FULL_NAME: &'static str;

    fn get_name() -> String;

    fn get_value(table: &Self::Table) -> Self::Type;
}

/// Trait that is used for compile time check, of availability of the column in table group
pub trait Allowed<T> {}

/// Compile time check
pub fn check_func<T: Column, Type>()
where
    Type: Allowed<<T as Column>::Table>
{

}
#[derive(Debug, Clone, Default)]
pub struct RawColumn {
    pub table_name: String,
    pub name: String,
}

impl AutoQueryable for RawColumn{}

impl Queryable for RawColumn {
    fn convert_to_query(&self) -> Option<String> {
        Some(format!("{}.{}", self.table_name, if self.table_name.len() > 0 {self.name.clone()} else {"".to_string()}))
    }
}


mod column_tests {
    use crate::safe_expressions::SafeExpr;
    use crate::literals::Null;
    use crate::convertible::ConvertibleTo;
    use my_macros::table;

    #[table]
    struct some_table {
        #[sql_type(Int)]
        #[column]
        #[null]
        UsersColumn: i32,
    }

    fn exclude_braces(mut query: String) -> String {
        query.replace("(", "").replace(")", "")
    }

    #[test]
    fn test_column() {
        let safe_expr: SafeExpr<_, some_table> = SafeExpr::<UsersColumn, _>::column().add(SafeExpr::literal(10));

        println!("{}", safe_expr.expr.clone().to_query());

        assert_eq!("some_table.UsersColumn + 10", exclude_braces(safe_expr.expr.to_query()));
    }

    #[test]
    fn null_expressions() {
        let expr: SafeExpr<_, some_table> = SafeExpr::<UsersColumn, _>::column().is_null();

        //let wrong_expr: SafeExpr<_, ()> = SafeExpr::basic(10).is_null();

        println!("{}", expr.expr.to_query())
    }
}


