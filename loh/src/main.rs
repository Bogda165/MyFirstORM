use std::collections::HashMap;
use std::os::macos::raw::stat;
use rusqlite::{Connection, OpenFlags, Row, Statement};

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct TableName {
    name: String
}

#[derive(Debug)]

struct EntityQuery<'a> {
    pub query: Statement<'a>,
    pub entity_queries: HashMap<TableName, EntityQuery<'a>>
}

impl EntityQuery<'_> {
    pub fn load<T: DbResponseConv>(&mut self, id: Option<String>) -> impl Iterator<Item = T> {
        let clos = |row: &Row| {
            let mut obj = T::from_response(row);

            self.entity_queries.iter_mut().for_each(|(table_name, query)| {
                obj.fill_fk(table_name, query);
            });

            Ok(obj)
        };
        match id {
            None => {
                self.query.query_map( [], clos).unwrap().map(|result| {
                    result.unwrap()
                }).collect::<Vec<T>>()
            }
            Some(id) => {
                self.query.query_map( &[(":id", id.as_str())], clos).unwrap().map(|result| {
                    result.unwrap()
                }).collect::<Vec<T>>()
            }
        }.into_iter()

    }
}

trait DbResponseConv {
    fn from_response(row: &Row) -> Self;
    fn fill_fk(&mut self, table_name: &TableName, query_table: &mut EntityQuery<'_>);
}

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
    posts: Vec<Post>
}

impl DbResponseConv for User {
    fn from_response(row: &Row) -> Self {
        User {
            user_id: row.get::<&str, i32>("user_id").unwrap(),
            name: row.get::<&str, String>("name").unwrap(),
            posts: vec![],
        }
    }

    fn fill_fk(&mut self, table_name: &TableName, query_table: &mut EntityQuery<'_>) {
        match table_name.name.as_str() {
            "posts" => {
                let posts: Vec<Post> = query_table.load(Some(self.user_id.to_string())).collect();
                self.posts.extend(posts);
            }
            _ => {
                panic!("Unknown name of the fields");
            }
        }
    }
}

#[derive(Debug)]
struct Post {
    post_id: i32,
    user_id: i32,
    content: String
}

impl DbResponseConv for Post {
    fn from_response(row: &Row) -> Self {
        Post {
            post_id: row.get:: <&str, i32>("post_id").unwrap(),
            user_id: row.get::<&str, i32>("user_id").unwrap(),
            content: row.get::<&str, String>("content").unwrap(),
        }
    }

    fn fill_fk(&mut self, table_name: &TableName, query_table: &mut EntityQuery<'_>) {
        //empty
    }
}

fn main() {

    let conn =  Connection::open_with_flags(
        "../test_db2.sqlite",
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    ).unwrap();

    let statement = conn.prepare("select * from Users").unwrap();

    let mut eq = EntityQuery {
        query: statement,
        entity_queries: HashMap::new(),
    };

    let statement2 = conn.prepare("select * from Posts where user_id = :id").unwrap();
    eq.entity_queries.insert(TableName { name: "posts".to_string() }, EntityQuery {
        query: statement2,
        entity_queries: HashMap::new(),
    });


    let users: Vec<User> = eq.load(None).collect();

    println!("{:?}", users);
}