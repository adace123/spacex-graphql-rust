use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct HistoryQueryOptions {
    pub id: Option<i32>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub flight_number: Option<i32>,
}

impl BaseQueryOptions for HistoryQueryOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("id", self.id.map(|i| i.to_string())),
            ("start", self.start),
            ("end", self.end),
            ("flight_number", self.flight_number.map(|f| f.to_string())),
        ];
        build_querystring(fields)
    }
}
