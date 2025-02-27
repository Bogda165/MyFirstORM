pub mod expressions;
pub mod column;
pub mod literals;
pub mod operators;
pub mod queryable;
pub mod safe_expressions;
pub mod convertible;
pub mod query;
pub mod raw_column;
pub use my_macros::*;


mod tables {
    use crate::column::Allowed;
    use super::*;
    pub mod users {

        use rusqlite::types::FromSqlResult;
        use rusqlite::types::ValueRef;
        use my_macros::table;

        #[table]
        struct users {
            #[column]
            #[sql_type(Int)]
            id: i32,
            #[column]
            #[sql_type(Text)]
            name: String,
        }
    }

    pub mod address {

        use rusqlite::types::FromSqlResult;
use rusqlite::types::ValueRef;
use crate::table;

        #[table]
        struct address {
            #[column]
            #[sql_type(Int)]
            id: i32,
            #[column]
            #[sql_type(Text)]
            street: String,
        }
    }

    pub mod phone {
        use rusqlite::types::FromSqlResult;
use rusqlite::types::ValueRef;
use super::*;

        #[table]
        struct phone {
            #[column]
            #[sql_type(Int)]
            id: i32,
            #[column]
            #[sql_type(Int)]
            number: i32,
        }
    }

    from!(phone::phone, users::users, address::address);
}

use tables::*;
use crate::safe_expressions::{column, literal};
use crate::query::the_query::Query;
use crate::queryable::Queryable;

#[test]
fn main_test() {
    let query = query_from!(users::users, address::address)
        .join::<phone::phone>(
            literal(10).less(column(phone::id))
                .and(
                    column(phone::number)
                        .cast::<i32>()
                        .div(literal(1000))
                        .equal(literal(9898))
                )
        ).where_clause(
        column(users::name)
            .like("%ll", Some(' '))
    ).select_test(
        ((column(users::name), "name"),
         ((column(phone::number),"number"),
          (column(address::street), "street")))
    );



    assert_eq!("{}", query.to_query());
}

fn main() {

}
