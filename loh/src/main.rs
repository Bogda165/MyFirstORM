use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::os::macos::raw::stat;
use rusqlite::{Connection, OpenFlags, Row, Statement, ToSql};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::types::Value::Null;
use crate::DbTypes::*;

trait INSERTABLE {
    fn for_insert(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum HUI<T> {
    NULL,
    VALUE(T)
}

impl<T> ToSql for HUI<T>
where T: ToSql
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            HUI::NULL => {Null.to_sql()}
            HUI::VALUE(val) => {
                val.to_sql()
            }
        }
    }
}

impl FromSql for HUI<i32> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Ok(HUI::NULL)
            }
            ValueRef::Integer(val) => {
                //TODO fix to i64 uoy
                Ok(HUI::VALUE(val as i32))
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

impl FromSql for HUI<f32> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Ok(HUI::NULL)
            }
            ValueRef::Real(val) => {
                Ok(HUI::VALUE(val as f32))
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

impl FromSql for HUI<String> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Ok(HUI::NULL)
            }
            ValueRef::Text(val) => {
                Ok(HUI::VALUE(std::str::from_utf8(val).unwrap().to_string()))
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }
}


#[derive(Debug)]
pub enum DbTypes {
    INTEGER_N(HUI<i32>),
    FLOAT_N(HUI<f32>),
    TEXT_N(HUI<String>),
    INTEGER(i32),
    FLOAT(f32),
    TEXT(String),
}

impl DbTypes {
    pub fn to_string(&self) -> String {
        match &self {
            INTEGER(_) => {
                "INTEGER".to_string()
            }
            FLOAT(_) => {
                "REAL".to_string()
            }
            TEXT(_) => {
                "TEXT".to_string()
            }
            INTEGER_N(_) => {
                "INTEGER".to_string()
            }
            FLOAT_N(_) => {
                "REAL".to_string()
            }
            TEXT_N(_) => {
                "TEXT".to_string()
            }

            _ => {
                unreachable!("I hae not any others types")
            }
        }
    }

    pub fn get_val(&self) -> String {
        let _str = format!("{:?}", self);
        let slice = _str.find("(").unwrap() + 1;
        let slice_2 = _str.len() - 1;
        _str[slice..slice_2].to_string()
    }
}

impl INSERTABLE for DbTypes {
    fn for_insert(&self) -> String {
        match self {
            INTEGER(val) => {
                val.to_string()
            }
            FLOAT(val) => {
                val.to_string()
            }
            TEXT(str) => {
                str.clone()
            }
            INTEGER_N(val) => {
                match val {
                    HUI::NULL => {"NULL".to_string()}
                    HUI::VALUE(val) => {val.to_string()}
                }
            }
            FLOAT_N(val) => {
                match val {
                    HUI::NULL => {"NULL".to_string()}
                    HUI::VALUE(val) => {val.to_string()}
                }
            }
            TEXT_N(val) => {
                match val {
                    HUI::NULL => {"NULL".to_string()}
                    HUI::VALUE(val) => {val.clone()}
                }
            }
        }
    }
}


impl ToSql for DbTypes {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        match self {
            DbTypes::INTEGER(i) => {Ok(ToSqlOutput::from(*i))},
            DbTypes::FLOAT(f) => Ok(ToSqlOutput::from(*f)),
            DbTypes::TEXT(s) => Ok(ToSqlOutput::from(s.clone())),
            INTEGER_N(i) => {i.to_sql()},
            FLOAT_N(f) => {f.to_sql()},
            TEXT_N(t) => {t.to_sql()},
        }
    }
}

impl FromSql for DbTypes {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => {
                Err(FromSqlError::InvalidType)
            }
            ValueRef::Integer(val) => {
                Ok(INTEGER(val as i32))
            }
            ValueRef::Real(val) => {
                Ok(FLOAT(val as f32))
            }
            ValueRef::Text(val) => {
                Ok(TEXT(std::str::from_utf8(val).unwrap().to_string()))
            }
            ValueRef::Blob(_) => { unreachable!("Fuck it") }
        }
    }
}

#[derive(Debug)]
struct ConnectionTable {
    base_id: String,
    upper_id: String,
    table: HashMap<String, Vec<String>>
}

impl ConnectionTable {
    pub fn new(base_id: &str, upper_id: &str) -> Self {
        ConnectionTable {
            base_id: base_id.to_string(),
            upper_id: upper_id.to_string(),
            table: HashMap::<String, Vec<String>>::new(),
        }
    }
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq, Clone)]
struct TableName {
    name: String
}


#[derive(Debug)]
enum LoadType {
    Lazy {addition_query: String},
    PreLoad{ table: Vec<Box<dyn DbResponseConv>>, connection_table: Option<ConnectionTable>},
}

#[derive(Debug)]
struct EntityQuery2<'a> {
    pub query: Statement<'a>,
    pub load: LoadType,
    pub entity_queries: HashMap<TableName, EntityQuery2<'a>>
}


impl EntityQuery2<'_> {
    fn pre_load(&mut self, entity: Box<dyn DbResponseConv>) {
        match &mut self.load {
            LoadType::Lazy{..} => {
                println!{"no need to preload lzy loading"};
            }
            LoadType::PreLoad {table, connection_table} => {
                let mut create_cb = match connection_table {
                    None => {None}
                    Some(connection_table) => {
                        Some(|row: &Row| {
                            connection_table.table.insert(
                                row.get::<&str, DbTypes>(connection_table.upper_id.as_str()).unwrap().for_insert(),
                                vec![
                                    row.get::<&str, DbTypes>(connection_table.base_id.as_str()).unwrap().for_insert()
                                ]
                            );
                        })
                    }
                };
                let mut ids: Vec<i32> = vec![];
                let clos = |row: &Row| {
                    let id = row.get::<&str, i32>("__rw").unwrap();
                    if ids.iter().find(|_id| {
                        **_id == id
                    }).is_some() {
                        return Err(rusqlite::Error::ExecuteReturnedResults);
                    }
                    ids.push(id);
                    match &mut create_cb{
                        None => {}
                        Some(closure) => {closure(row)}
                    }
                    Ok(entity.from_response(row))
                };

                *table = self.query.query_map( [], clos).unwrap().filter(|result| {
                    match result {
                        Ok(_) => {true}
                        Err(_) => {false}
                    }
                }).map(|result| {
                    result.unwrap()
                }).collect::<Vec<Box<dyn DbResponseConv>>>();
            }
            _ => {unreachable!()}
        }
    }

    // fn load<Entity>(self) -> impl Iterator<Item = Box<dyn DbResponseConv>>
    // where
    //     Entity: DbResponseConv + Default
    // {
    //     panic!()
    // }

}

/*
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

 */

trait DbResponseConv: std::fmt::Debug + Any {
    fn default_obj(&self) -> Box<dyn DbResponseConv>;
    fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv>;
    //fn fill_fk(&mut self, table_name: &TableName, query_table: &mut EntityQuery2<'_>);

    fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2);

    fn pre_load(&mut self, eq: &mut EntityQuery2) {
        eq.pre_load(self.default_obj());
        eq.entity_queries.iter_mut().for_each(|(tb, eq)| {
            self.for_every(Box::new(some_function), tb, eq);
        });
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    fn get_by_name(&self, name: String) -> DbTypes;

    fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>);

    fn clone_box(&self) -> Box<dyn DbResponseConv>;

    //fn load(_vec: Vec<Box<dyn DbResponseConv>>, tb: TableName, eq: EntityQuery2) -> Vec<Box<Self>>;
}

pub fn load1(){
    //load -> result a vector of Entity
    //go througu eq map for every laod<Entity>
}

pub fn load(mut collection: &mut Option<Vec<Box<dyn DbResponseConv>>>, table_name: TableName, eq: EntityQuery2)
{
    if collection.is_none() {
        *collection = match eq.load {
            LoadType::PreLoad { table, connection_table } => {
                Some(table)
            }
            LoadType::Lazy { .. } => {
                unreachable!()
            }
        };
        eq.entity_queries.into_iter().for_each(|(inside_tb, inside_eq)| {
            load(collection, inside_tb, inside_eq)
        });
        return;
    }

    if let Some(collection) = collection.as_mut() {
        match eq.load {
            LoadType::PreLoad { mut table, connection_table } => {

                let mut table = Some(table);

                eq.entity_queries.into_iter().for_each(|(inside_tb, inside_eq)| {
                    load(&mut table, inside_tb, inside_eq)
                });

                if let Some(table) = table.as_mut() {
                    match connection_table {
                        Some(mut connection_table) => {
                            collection.iter_mut().for_each(|obj| {
                                let connected_with = obj.get_by_name(connection_table.upper_id.clone()).get_val();

                                if let Some(_vec) = connection_table.table.remove(&connected_with) {
                                    let clean_vec = table.iter().filter(|obj| {
                                        return _vec.iter().find(|val| {
                                            obj.get_by_name(connection_table.base_id.clone()).for_insert() == **val
                                        }).is_some()
                                    }).collect();

                                    obj.add(table_name.clone(), clean_vec);
                                }else {
                                    println!("the elemnt was not connected to any fk");
                                }

                            });

                        }
                        None => {
                            println!("The end of recursion");
                        }
                    }
                }
            }
            LoadType::Lazy { .. } => {
                unreachable!{}
            }
        }
    }
}


fn some_function(mut type_e: Box<dyn DbResponseConv>, eq: &mut EntityQuery2) {
    type_e.pre_load(eq);
}

#[derive(Debug, Default, Clone)]
struct Author {
    author_id: i32,
    name: String,
    books: Vec<Book>
}

impl Author {

}

#[derive(Debug, Default, Clone)]
struct Book {
    book_id: i32,
    title: String,
    authors: Vec<Author>
}

impl Book {

}

impl DbResponseConv for Author {
    fn get_by_name(&self, name: String) -> DbTypes{
        match name.as_str() {
            "author_id" => DbTypes::INTEGER(self.author_id),
            "name" => DbTypes::TEXT(self.name.clone()),
            _ => {unreachable!("you")}
        }
    }
    fn default_obj(&self) -> Box<dyn DbResponseConv> {
        Box::new(Author::default())
    }
    fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
        println!("{:?}", row);
        Box::new(Author {
            author_id: row.get::<&str, i32>("author_id").unwrap(),
            name: row.get::<&str, String>("name").unwrap(),
            books: vec![],
        })
    }
    // fn fill_fk(&mut self, table_name: &TableName, query_table: EntityQuery2<'_>) {
    //
    //     match table_name.name.as_str() {
    //         "books" => {
    //             let books: Vec<Book> = query_table.load().collect();
    //
    //             self.books.extend(books);
    //         }
    //         _ => {}
    //     }
    //
    // }

    fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
        match tb.name.as_str() {
            "books" =>  {
                func(Box::new(Book::default()), eq);
            }
            _ => {
                panic!("Incorrect name of a table");
            }
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
        match table_name.name.as_str() {
            "books" =>  {
                let books: Vec<Book> = vector.into_iter().map(|obj| {
                    let any_obj = obj.clone_box().into_any();
                    let book = any_obj.downcast::<Book>().unwrap();
                    *book
                }).collect();

                self.books.extend(books);
            }
            _ => {
                panic!("Incorrect name of a table");
            }
        }
    }

    fn clone_box(&self) -> Box<dyn DbResponseConv> {
        Box::new(self.clone())
    }

    // fn load(_vec: Vec<Box<dyn DbResponseConv>>, tb: TableName, eq: EntityQuery2){
    //     let mut table = _vec.into_iter().map(|obj| {
    //         let any = obj.into_any();
    //         let hui = any.downcast::<Self>().unwrap();
    //         *hui
    //     }).collect();
    //
    //     eq.entity_queries.into_iter().for_each(|(tb, eq)| {
    //         table = load::<Self, Book>(table, tb, eq);
    //     });
    // }

}


impl DbResponseConv for Book {

    fn get_by_name(&self, name: String) -> DbTypes{
        match name.as_str() {
            "book_id" => DbTypes::INTEGER(self.book_id),
            "title" => DbTypes::TEXT(self.title.clone()),
            _ => {unreachable!("you")}
        }
    }
    fn default_obj(&self) -> Box<dyn DbResponseConv> {
        Box::new(Book::default())
    }
    fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
        Box::new(Book {
            book_id: row.get::<&str, i32>("book_id").unwrap(),
            title: row.get::<&str, String>("title").unwrap(),
            authors: vec![],
        })
    }

    // fn fill_fk(&mut self, table_name: &TableName, query_table: &mut EntityQuery2<'_>) {
    //     unreachable!()
    //     /*
    //     match table_name.name.as_str() {
    //         "authors" => {
    //             let authors: Vec<Author> = query_table.load(Some(self.book_id.to_string())).collect();
    //             self.authors.extend(authors);
    //         }
    //         _ => {}
    //     }
    //      */
    // }

    fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
        match tb.name.as_str() {
            "authors" =>  {
                func(Box::new(Book::default()), eq);
            }
            _ => {
                panic!("Incorrect name of a table");
            }
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {

    }

    fn clone_box(&self) -> Box<dyn DbResponseConv> {
        Box::new(self.clone())
    }


}


/*
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

#[test]
fn test1() {

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
*/

#[test]
fn test2 () {
    let conn =  Connection::open_with_flags(
        "../test_db2.sqlite",
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    ).unwrap();

    let statement = conn.prepare("select ROWID as __rw * from Authors").unwrap();

    let mut eq = EntityQuery2 {
        query: statement,
        load: LoadType::PreLoad{table: vec![], connection_table: None},
        entity_queries: HashMap::new(),
    };

    let statement2 = conn.prepare("select Books.ROWID as __rw * from Books, Author_Book where Author_Book.book_id = Books.book_id").unwrap();
    eq.entity_queries.insert(TableName { name: "books".to_string() }, EntityQuery2 {
        query: statement2,
        load: LoadType::PreLoad{table: vec![], connection_table: Some(ConnectionTable::new("book_id", "author_id"))},
        entity_queries: HashMap::new(),
    });

    let mut author = Author::default();
    author.pre_load(&mut eq);


    println!{"{:?}", eq};
}



fn main() {

    let conn =  Connection::open_with_flags(
        "../test_db2.sqlite",
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    ).unwrap();

    let statement = conn.prepare("select ROWID as __rw, * from Authors").unwrap();

    let mut eq = EntityQuery2 {
        query: statement,
        load: LoadType::PreLoad{table: vec![], connection_table: None},
        entity_queries: HashMap::new(),
    };

    let statement2 = conn.prepare("select Books.ROWID as __rw, * from Books, Author_Book where Author_Book.book_id = Books.book_id").unwrap();
    eq.entity_queries.insert(TableName { name: "books".to_string() }, EntityQuery2 {
        query: statement2,
        load: LoadType::PreLoad{table: vec![], connection_table: Some(ConnectionTable::new("book_id", "author_id"))},
        entity_queries: HashMap::new(),
    });

    let mut author = Author::default();

    author.pre_load(&mut eq);

    println!("{:?}", eq);

    let mut authors = None;

    load(&mut authors,TableName{ name: "authors".to_string() }, eq);
    println!("\n\n\n");
    println!{"{:?}", authors};
}