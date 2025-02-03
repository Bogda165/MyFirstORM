use my_macros::{AutoQueryable, Queryable};
use crate::create_a_name::Queryable;
use crate::expressions::Expression;
use crate::safe_expressions::SafeExpr;

/// trait that should implement every table, impl with #[table] macro
pub trait Table {}

impl Table for () {}

/// Trait of a column in the table, impl using #[table] macro, and attribute #[column]
pub trait Column {
    /// Type of the columns table
    type Table;
    type Type;

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


