use sql_query::*;
mod tables {
    use sql_query::column::Allowed;
    use super::*;
    use sql_query::table;
    pub mod users {
        use super::*;
        #[table]
        struct users {
            #[column]
            pub id: i32,
            #[column]
            pub name: String,
        }
    }

    pub mod address {
        use super::*;
        #[table]
        struct address {
            #[column]
            pub id: i32,
            #[column]
            pub street: String,
        }
    }

    pub mod phone {
        use super::*;
        #[table]
        struct phone {
            #[column]
            pub id: i32,
            #[column]
            pub number: i32,
        }
    }

    from!(phone::phone, users::users, address::address);
}

use tables::*;
use sql_query::safe_expressions::{column, literal};
use sql_query::query::the_query::Query;
use sql_query::queryable::Queryable;

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

fn main() {}