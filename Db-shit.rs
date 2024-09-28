enum DbTypes {
    INTEGER(i32),
    FLOAT(f32),
    TEXT(String),
}

pub struct User {
    id: i32,
    name: String,
}

pub struct UserTable {
    id: (DbTypes, Attributes, Attributes),
    name: DbTypes,
}

impl User {
    pub fn get_table(&self) -> UserTable {
        UserTable {
            id: (DbTypes::INTEGER(self.id), Attributes::PK, Attributes::AUTO_I),
            name: DbTypes::TEXT(self.name.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_table() {
        let user_instance = User {
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