use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct MetaData <'a>{
    pub attr_type: HashMap<&'a str, &'a str>,
}
impl<'a> MetaData<'a> {
    pub fn default() -> MetaData<'a> {
        let mut set = HashMap::new();
        set.insert("INTEGER",  "INTEGER");
        set.insert( "FLOAT",  "REAL");
        set.insert( "TEXT",  "TEXT");
        set.insert( "PK",  "PRIMARY KEY");
        set.insert( "AUTO_I",  "AUTOINCREMENT");
        set.insert("INTEGER_N", "INTEGER");
        set.insert("FLOAT_N", "FLOAT");
        set.insert("TEXT_N", "TEXT");
        set.insert( "CONNECT", "");
        MetaData {
            attr_type: set
        }
    }
}