use crate::column::Column;

struct Query {
    columns: Vec<Column>
}

impl Query {
    pub fn select(self) {
        unreachable!()
    }
}

pub trait Queryable {
    fn convert_to_query(&self) -> Option<String>;

    /// do not re-implement by yourself
    fn to_query_auto(&self) -> Option<String> {
        None
    }

    fn to_query(&self) -> String {
        if let Some(query) = self.convert_to_query() {
            query
        } else {
            self.to_query_auto()
                .expect("Nor impl with macro, explicit impl was provided")
        }
    }
}