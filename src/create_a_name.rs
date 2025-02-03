use crate::column::Column;

/// Implement this trait with derive macro if you want to generate auto transformation into query. Read mor ein macro
pub trait AutoQueryable {
    fn to_query_auto(&self) -> Option<String> {
        None
    }
}

/// Implement this create wiht derive masro, explisitly impl it, if you want to expand implementation
pub trait Queryable: AutoQueryable {
    fn convert_to_query(&self) -> Option<String>;

    fn to_query(&self) -> String {
        if let Some(query) = self.convert_to_query() {
            query
        } else {
            self.to_query_auto()
                .expect("Nor impl with macro, explicit impl was provided")
        }
    }
}