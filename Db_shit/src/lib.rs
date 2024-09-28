use macros_l::*;
use crate::Attributes::{AUTO_I, PK};
use crate::DbTypes::{INTEGER, TEXT};

#[fields_name]
#[derive(Debug)]
pub enum DbTypes {
    INTEGER(i32),
    FLOAT(f32),
    TEXT(String)
}
#[derive(Debug)]
#[fields_name]
pub enum Attributes {
    PK,
    AUTO_I,
    CONNECT(&'static str)
}

pub struct user {
    //INTEGER, PK, AUTO_I
    pub id: i32,
    //TEXT
    pub name: String,
}

pub struct _user {
    pub id: (DbTypes, Attributes, Attributes),
    pub name: (DbTypes)
}
/* build a table of the struct
           "field1": "attr1", "attr2" ...
           ...
        */
impl user {
    pub fn get_table(&self) -> _user{
        _user {
            id: (INTEGER(self.id), PK, AUTO_I),
            name: TEXT(self.name.clone()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_table() {
        let user_instance = user {
            id: 1,
            name: "Alice".to_string(),
        };

        let table = user_instance.get_table();

        match table.id {
            (DbTypes::INTEGER(val), Attributes::PK, Attributes::AUTO_I) => assert_eq!(val, 1),
            _ => panic!("ID field did not match expected values"),
        }

        match table.name {
            DbTypes::TEXT(val) => assert_eq!(val, "Alice".to_string()),
            _ => panic!("Name field did not match expected values"),
        }
    }
}