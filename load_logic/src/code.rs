use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use rusqlite::{Row, Statement};
use rusqlite::types::{FromSql, Value};
use crate::{INSERTABLE};
use std::fmt;

#[derive(Debug)]
pub struct ConnectionTable {
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
pub struct TableName {
    pub name: String
}

pub enum LoadType {
    Lazy { addition_query: String },
    PreLoad { table: Vec<Box<dyn DbResponseConv>>, connection_table: Option<ConnectionTable> },
}

pub struct EntityQuery2<'a> {
    pub query: Statement<'a>,
    pub load: LoadType,
    pub entity_queries: HashMap<TableName, EntityQuery2<'a>>
}


impl EntityQuery2<'_> {
    fn pre_load(&mut self, entity: Box<dyn DbResponseConv>) {
        match &mut self.load {
            LoadType::Lazy { .. } => {
                println! {"no need to preload lzy loading"};
            }
            LoadType::PreLoad { table, connection_table } => {
                let mut create_cb = match connection_table {
                    None => { None }
                    Some(connection_table) => {
                        Some(|row: &Row| {
                            connection_table.table
                                .entry(
                                    format!("{:?}", row.get_ref::<&str>(connection_table.upper_id.as_str())
                                    .unwrap())
                                )
                                .or_insert(vec![])
                                .push(
                                    format!("{:?}", row.get_ref::<&str>(connection_table.base_id.as_str())
                                    .unwrap())
                                )
                        })
                    }
                };

                let mut ids: Vec<i32> = vec![];

                let clos = |row: &Row| {

                    match &mut create_cb {
                        None => {}
                        Some(closure) => { closure(row) }
                    }

                    let id = row.get::<&str, i32>("__rw").unwrap();
                    if ids.iter().find(|_id| {
                        **_id == id
                    }).is_some() {
                        println!("The same id met twice");
                        return Err(rusqlite::Error::ExecuteReturnedResults);
                    }
                    ids.push(id);

                    Ok(entity.from_response(row))
                };

                *table = self.query.query_map([], clos).unwrap().filter(|result| {
                    match result {
                        Ok(_) => { true }
                        Err(_) => { false }
                    }
                }).map(|result| {
                    result.unwrap()
                }).collect::<Vec<Box<dyn DbResponseConv>>>();
            }
            _ => { unreachable!() }
        }
    }

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

pub trait DbResponseConv: Any {
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

    fn get_by_name(&self, name: &String) -> Value;

    fn add(&mut self, table_name: TableName, vector: Vec<&Box<dyn DbResponseConv>>);

    fn clone_box(&self) -> Box<dyn DbResponseConv>;

    //fn load(_vec: Vec<Box<dyn DbResponseConv>>, tb: TableName, eq: EntityQuery2) -> Vec<Box<Self>>;
}

pub fn load1() {
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
                                let connected_with = format!("{:?}", obj.get_by_name(&connection_table.upper_id));

                                if let Some(_vec) = connection_table.table.remove(&connected_with) {
                                    let clean_vec = table.iter().filter(|obj| {
                                        return _vec.iter().find(|val| {
                                            format!("{:?}", obj.get_by_name(&connection_table.base_id)) == **val
                                        }).is_some()
                                    }).collect();

                                    obj.add(table_name.clone(), clean_vec);
                                }else {
                                    println!("the elemnt was not connected to any fk {connected_with}");
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

fn new_func() {
    
}
