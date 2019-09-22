use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct HistoryQueryOptions {
    pub id: Option<i32>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub flight_number: Option<i32>,
}
