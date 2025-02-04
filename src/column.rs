use crate::convertible::TheType;
use crate::create_a_name::Queryable;
use crate::expressions::RawTypes;

/// trait that should implement every table, impl with #[table] macro
pub trait Table {}

impl Table for () {}

/// Trait of a column in the table, impl using #[table] macro, and attribute #[column]
pub trait Column: Default + TheType + Into<RawTypes>{
    /// Type of the columns table
    type Table;

    fn get_name() -> String;
}

/// Trait that is used for compile time check, of availability of the column in table group
pub trait Allowed<T> {}

/// Compile time check
pub fn check_func<T: Column, Type>()
where
    Type: Allowed<<T as Column>::Table>
{

}

struct RawColumn;

mod column_tests {
    use super::{Allowed, Column};
    use crate::literals::*;
    use crate::convertible::*;
    use crate::{conversation, convertible, self_converted, RawColumn};
    use crate::create_a_name::Queryable;
    use crate::expressions::RawTypes;
    use crate::safe_expressions::SafeExpr;

    // let it be i32 type column
    struct UsersColumn;

    impl Allowed<()> for () {}

    impl Default for UsersColumn {
        fn default() -> Self {
            UsersColumn {}
        }
    }

    impl TheType for UsersColumn {
        type Type = i32;
    }

    impl Column for UsersColumn {
        type Table = ();

        fn get_name() -> String {
            unreachable!();
        }
    }


    impl Into<RawTypes> for UsersColumn {
        fn into(self) -> RawTypes {
            RawTypes::Column(RawColumn{ table_name: "".to_string(), name: "users_column".to_string() })
        }
    }


    fn exclude_braces(mut query: String) -> String {
        query.replace("(", "").replace(")", "")
    }

    #[test]
    fn test_column() {
        let safe_expr: SafeExpr<_, ()> = SafeExpr::<UsersColumn, _>::column().add(SafeExpr::basic(10));

        println!("{}", safe_expr.expr.clone().to_query());

        assert_eq!("users_column + 10", exclude_braces(safe_expr.expr.to_query()));
    }

}


