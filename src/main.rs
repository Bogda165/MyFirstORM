use crate::query::the_query::Query;
use std::fmt::format;
use my_macros::{table};
use crate::column::Column;
use crate::queryable::{AutoQueryable, Queryable};

mod expressions;
mod column;
mod literals;
mod operators;
mod queryable;
mod safe_expressions;
mod convertible;
mod query;
mod raw_column;

mod tables {
    use super::*;
    use my_macros::{from, table};
    use crate::queryable::Queryable;
    use crate::column::Allowed;
    use crate::column::Table;
    use crate::column::Column;
    use crate::convertible::TheType;
    pub mod users {
        use crate::column::RawColumn;
use crate::expressions::raw_types::RawTypes;
use super::*;

        #[table]
        struct users {
            #[column]
            id: i32,
            #[column]
            name: String,
        }
    }

    pub mod address {
        use crate::column::RawColumn;
use crate::expressions::raw_types::RawTypes;
use super::*;

        #[table]
        struct address {
            #[column]
            id: i32,
            #[column]
            street: String,
        }
    }

    pub mod phone {
        use crate::column::RawColumn;
use crate::expressions::raw_types::RawTypes;
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



fn main() {
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
