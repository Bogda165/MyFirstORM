mod code;

use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::ErrorKind::Interrupted;
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


mod many_to_many {
    use std::any::Any;
    use std::collections::HashMap;
    use rusqlite::{Connection, OpenFlags, Row};
    use crate::code::{load, ConnectionTable, DbResponseConv, EntityQuery2, LoadType, TableName};
    use crate::DbTypes;

    #[derive(Debug, Default, Clone)]
    struct Author {
        author_id: i32,
        name: String,
        books: Vec<Book>
    }

    impl Author {}

    #[derive(Debug, Default, Clone)]
    struct Book {
        book_id: i32,
        title: String,
        authors: Vec<Author>
    }

    impl Book {}

    impl DbResponseConv for Author {
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
        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                "books" => {
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

        fn get_by_name(&self, name: &String) -> DbTypes {
            match name.as_str() {
                "author_id" => DbTypes::INTEGER(self.author_id),
                "name" => DbTypes::TEXT(self.name.clone()),
                _ => { unreachable!("you") }
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                "books" => {
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
    }


    impl DbResponseConv for Book {
        fn get_by_name(&self, name: &String) -> DbTypes {
            match name.as_str() {
                "book_id" => DbTypes::INTEGER(self.book_id),
                "title" => DbTypes::TEXT(self.title.clone()),
                _ => { unreachable!("you") }
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
                "authors" => {
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

    #[test]
    fn many_to_many() {
        let conn =  Connection::open_with_flags(
            "../../resourses/test_db2.sqlite",
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
}


mod one_to_many {
    use std::any::Any;
    use std::collections::HashMap;
    use rusqlite::{Connection, OpenFlags, Row};
    use crate::code::{load, ConnectionTable, DbResponseConv, EntityQuery2, LoadType, TableName};
    use crate::DbTypes;
    use crate::DbTypes::{INTEGER, TEXT};

    #[derive(Debug, Default, Clone)]
    struct User {
        user_id: i32,
        name: String,
        posts: Vec<Post>
    }

    impl DbResponseConv for User {
        fn default_obj(&self) -> Box<dyn DbResponseConv> {
            Box::new(User::default())
        }

        fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
            Box::new(User {
                user_id: row.get::<&str, i32>("user_id").unwrap(),
                name: row.get::<&str, String>("name").unwrap(),
                posts: vec![],
            })
        }

        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                "posts" => {
                    func(Box::new(Post::default()), eq)
                }
                _ => {
                    unreachable!()
                }
            }
        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn get_by_name(&self, name: &String) -> DbTypes {
            match name.as_str() {
                "user_id" => INTEGER(self.user_id),
                "name" => TEXT(self.name.clone()),
                _ => {
                    unreachable!()
                }
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                "posts" => {
                    let posts: Vec<Post> = vector.into_iter().map(|obj| {
                        let any_obj = obj.clone_box().into_any();
                        let post = any_obj.downcast::<Post>().unwrap();
                        *post
                    }).collect();

                    self.posts.extend(posts);
                }
                _ => {
                    panic!("Incorrect name of a table");
                }
            }
        }

        fn clone_box(&self) -> Box<dyn DbResponseConv> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug, Default, Clone)]
    struct Post {
        post_id: i32,
        user_id: i32,
        content: String
    }

    impl DbResponseConv for Post {
        fn default_obj(&self) -> Box<dyn DbResponseConv> {
            Box::new(Post::default())
        }

        fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
            Box::new(Post {
                post_id: row.get::<&str, i32>("post_id").unwrap(),
                user_id: row.get::<&str, i32>("user_id").unwrap(),
                content: row.get::<&str, String>("content").unwrap(),
            })
        }

        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                _ => unreachable!()
            }
        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn get_by_name(&self, name: &String) -> DbTypes {
            match name.as_str() {
                "user_id" => INTEGER(self.user_id),
                "content" => TEXT(self.content.clone()),
                "post_id" => INTEGER(self.post_id),
                _ => {
                    unreachable!()
                }
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            unreachable!()
        }

        fn clone_box(&self) -> Box<dyn DbResponseConv> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn test1() {
        let conn = Connection::open_with_flags(
            "../../resourses/test_db2.sqlite",
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        ).unwrap();

        let statement = conn.prepare("select ROWID as __rw, * from Users").unwrap();

        let mut eq = EntityQuery2 {
            query: statement,
            load: LoadType::PreLoad { table: vec![], connection_table: None },
            entity_queries: HashMap::new(),
        };

        let statement2 = conn.prepare("select ROWID as __rw, * from Posts").unwrap();
        // let statement2 = conn.prepare("select * from Posts where user_id = :id").unwrap();
        eq.entity_queries.insert(TableName { name: "posts".to_string() }, EntityQuery2 {
            query: statement2,
            load: LoadType::PreLoad {table: vec![], connection_table: Some(ConnectionTable::new("post_id", "user_id"))},
            entity_queries: HashMap::new(),
        });

        let mut user = User::default();

        user.pre_load(&mut eq);


        let mut users = None;

        load(&mut users,TableName{ name: "users".to_string() }, eq);

        println!("{:?}", users);
    }
}

mod hard_test {
    use std::any::Any;
    use std::collections::HashMap;
    use std::io::ErrorKind::{AddrNotAvailable, Interrupted};
    use std::ops::Deref;
    use rusqlite::{Connection, OpenFlags, Row};
    use rusqlite::ffi::sqlite3_busy_timeout;
    use crate::code::{load, ConnectionTable, DbResponseConv, EntityQuery2, LoadType, TableName};
    use crate::DbTypes;
    use crate::DbTypes::{INTEGER, TEXT};

    #[derive(Clone, Debug, Default)]
    struct Profile {
        profile_id: i32,
        student_id: i32,
        address: String,
        phone: String,
    }

    impl DbResponseConv for Profile {
        fn default_obj(&self) -> Box<dyn DbResponseConv> {
            Box::new(Profile::default())
        }

        fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
            Box::new(Profile {
                student_id: row.get::<&str, i32>("student_id").unwrap(),
                profile_id: row.get::<&str, i32>("profile_id").unwrap(),
                address: row.get::<&str, String>("address").unwrap(),
                phone: row.get::<&str, String>("phone").unwrap(),
            })
        }

        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                _ => {
                    println!("there no such field in entity as {}", tb.name);
                }
            }
        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn get_by_name(&self, name: &String) -> DbTypes {
            match name.as_str() {
                "profile_id" => INTEGER(self.profile_id),
                "student_id" => INTEGER(self.student_id),
                "address" => TEXT(self.address.clone()),
                "phone" => TEXT(self.phone.clone()),
                _ => {
                    unreachable!()
                }
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                _ => {
                    println!("there no such field in entity as {}", table_name.name);
                }
            }
        }

        fn clone_box(&self) -> Box<dyn DbResponseConv> {
            Box::new(self.clone())
        }
    }

    #[derive(Clone, Debug, Default)]
    struct Course {
        course_id: i32,
        name: String,
        credit: i32,
        department: Department,
        students: Vec<Student>
    }

    impl DbResponseConv for Course {
        fn default_obj(&self) -> Box<dyn DbResponseConv> {
            Box::new(Course::default())
        }

        fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
            Box::new(Course {
                course_id: row.get::<&str, i32>("course_id").unwrap(),
                name: row.get::<&str, String>("name").unwrap(),
                credit: row.get::<&str, i32>("credits").unwrap(),
                department: Default::default(),
                students: vec![],
            })
        }

        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                "departments" => {
                    func(Box::new(Department::default()), eq)
                }
                "students" => {
                    func(Box::new(Student::default()), eq)
                }
                _ => {
                    println!("there no such field in entity as {}", tb.name);
                }
            }

        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn get_by_name(&self, name: &String) -> DbTypes {
            match name.as_str() {
                "course_id" => {INTEGER(self.course_id)}
                "name" => TEXT(self.name.clone()),
                "credit" => INTEGER(self.credit),
                _ => {unreachable!()}
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                "departments" => {
                    assert_eq! {vector.len(), 1}

                    let [department]: [&Box<dyn DbResponseConv>; 1] = vector.try_into().unwrap();

                    let any_obj = department.clone_box().into_any();
                    let department = any_obj.downcast::<Department>().unwrap();

                    self.department = *department
                }
                "students" => {
                    let posts: Vec<Student> = vector.into_iter().map(|obj| {
                        let any_obj = obj.clone_box().into_any();
                        let post = any_obj.downcast::<Student>().unwrap();
                        *post
                    }).collect();

                    self.students.extend(posts);
                }
                _ => {
                    println!("there no such field in entity as {}", table_name.name);
                }
            }
        }

        fn clone_box(&self) -> Box<dyn DbResponseConv> {
            Box::new(self.clone())
        }
    }

    #[derive(Clone, Debug, Default)]
    struct Student {
        student_id: i32,
        name: String,
        age: i32,
        department: Department,
        courses: Vec<Course>,
        profile: Profile,
    }

    impl DbResponseConv for Student {
        fn default_obj(&self) -> Box<dyn DbResponseConv> {
            Box::new(Student::default())
        }

        fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
            Box::new(Student {
                student_id: row.get::<&str, i32>("student_id").unwrap(),
                name: row.get::<&str, String>("name").unwrap(),
                age: row.get::<&str, i32>("age").unwrap(),
                department: Default::default(),
                profile: Profile::default(),
                courses: vec![],
            })
        }

        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                "departments" => {
                    func(Box::new(Department::default()), eq)
                }
                "courses" => {
                    func(Box::new(Course::default()), eq)
                }
                "profiles" => {
                    func(Box::new(Profile::default()), eq)
                }
                _ => {
                    println!("there no such field in entity as {}", tb.name);
                }
            }

        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn get_by_name(&self, name: String) -> DbTypes {
            match name.as_str() {
                "student_id" => {INTEGER(self.student_id)}
                "name" => TEXT(self.name.clone()),
                "age" => INTEGER(self.age),
                _ => {unreachable!()}
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                "departments" => {
                    assert_eq! {vector.len(), 1}

                    let [department]: [&Box<dyn DbResponseConv>; 1] = vector.try_into().unwrap();

                    let any_obj = department.clone_box().into_any();
                    let department = any_obj.downcast::<Department>().unwrap();

                    self.department = *department
                }
                "courses" => {
                    let posts: Vec<Course> = vector.into_iter().map(|obj| {
                        let any_obj = obj.clone_box().into_any();
                        let post = any_obj.downcast::<Course>().unwrap();
                        *post
                    }).collect();

                    self.courses.extend(posts);
                }
                "profiles" => {
                    assert_eq! {vector.len(), 1}

                    let [department]: [&Box<dyn DbResponseConv>; 1] = vector.try_into().unwrap();

                    let any_obj = department.clone_box().into_any();
                    let department = any_obj.downcast::<Profile>().unwrap();

                    self.profile = *department
                }
                _ => {
                    println!("there no such field in entity as {}", table_name.name);
                }
            }
        }

        fn clone_box(&self) -> Box<dyn DbResponseConv> {
            Box::new(self.clone())
        }
    }

    #[derive(Clone, Debug, Default)]
    struct Department {
        department_id: i32,
        name: String
    }

    impl DbResponseConv for Department {
        fn default_obj(&self) -> Box<dyn DbResponseConv> {
            Box::new(Department::default())
        }

        fn from_response(&self, row: &Row) -> Box<dyn DbResponseConv> {
            Box::new(Department {
                department_id: row.get::<&str, i32>("department_id").unwrap(),
                name: row.get::<&str, String>("name").unwrap(),
            })
        }

        fn for_every(&self, func: Box<dyn Fn(Box<dyn DbResponseConv>, &mut EntityQuery2)>, tb: &TableName, eq: &mut EntityQuery2) {
            match tb.name.as_str() {
                _ => {
                    println!("there no such field in entity as {}", tb.name);
                }
            }
        }

        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }

        fn get_by_name(&self, name: String) -> DbTypes {
            match name.as_str() {
                "department_id" => INTEGER(self.department_id),
                "name" => TEXT(self.name.clone()),
                _ => {
                    unreachable!()
                }
            }
        }

        fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>) {
            match table_name.name.as_str() {
                _ => {
                    println!("there no such field in entity as {}", table_name.name);
                }
            }
        }

        fn clone_box(&self) -> Box<dyn DbResponseConv> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn hard_test () {
        let conn = Connection::open_with_flags(
            "../../resourses/test_db2.sqlite",
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        ).unwrap();

        let statement = conn.prepare("select ROWID as __rw, * from students").unwrap();

        let mut eq = EntityQuery2 {
            query: statement,
            load: LoadType::PreLoad { table: vec![], connection_table: None },
            entity_queries: HashMap::new(),
        };

        eq.entity_queries.insert(TableName {name: "departments".to_string()}, EntityQuery2{
            query: conn.prepare("select departments.ROWID as __rw, *, student_id from departments, students where students.department_id = departments.department_id").unwrap(),
            load: LoadType::PreLoad { table: vec![], connection_table: Some(ConnectionTable::new("department_id", "student_id"))},
            entity_queries: HashMap::new(),
        });

        eq.entity_queries.insert(TableName {name: "profiles".to_string()}, EntityQuery2{
            query: conn.prepare("select profiles.ROWID as __rw, * from profiles").unwrap(),
            load: LoadType::PreLoad { table: vec![], connection_table: Some(ConnectionTable::new("profile_id", "student_id"))},
            entity_queries: HashMap::new(),
        });

        eq.entity_queries.insert(TableName {name: "courses".to_string()}, EntityQuery2{
            query: conn.prepare("select courses.ROWID as __rw, * from courses, enrollments where enrollments.course_id = courses.course_id").unwrap(),
            load: LoadType::PreLoad { table: vec![], connection_table: Some(ConnectionTable::new("course_id", "student_id"))},
            entity_queries: HashMap::new(),
        });

        let mut students = Student::default();
        students.pre_load(&mut eq);

        let mut students = None;

        load(&mut students, TableName {name: "students".to_string()}, eq);

        let _ = students.is_some_and(|vec| {
            for dep in vec {
                println!("{:?}", dep);
            }
            true
        });

        // let statement2 = conn.prepare("select ROWID as __rw, * from Posts").unwrap();
        // // let statement2 = conn.prepare("select * from Posts where user_id = :id").unwrap();
        // eq.entity_queries.insert(TableName { name: "posts".to_string() }, EntityQuery2 {
        //     query: statement2,
        //     load: LoadType::PreLoad {table: vec![], connection_table: Some(ConnectionTable::new("post_id", "user_id"))},
        //     entity_queries: HashMap::new(),
        // });
        //
        // let mut user = crate::one_to_many::User::default();
        //
        // user.pre_load(&mut eq);
        //
        //
        // let mut users = None;
        //
        // load(&mut users,TableName{ name: "users".to_string() }, eq);
        //
        // println!("{:?}", users);
    }
}



fn main() {

}