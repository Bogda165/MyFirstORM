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


        use crate::table;

        #[table]
        struct users {
            #[column]
            id: i32,
            #[column]
            name: String,
        }
    }

    pub mod address {

        use crate::table;

        #[table]
        struct address {
            #[column]
            id: i32,
            #[column]
            street: String,
        }
    }

    pub mod phone {
        use super::*;

        #[table]
        struct phone {
            #[column]
            id: i32,
            #[column]
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
fn test1() {
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
        (column(users::name),
         (column(phone::number),
          column(address::street)))
    );



    println!("{}", query.to_query());
}

fn main() {

}
