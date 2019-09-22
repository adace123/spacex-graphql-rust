use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct CoreQueryOptions {
    pub core_serial: Option<String>,
    pub block: Option<i32>,
    pub capsule_id: Option<String>,
    pub status: Option<String>,
    pub original_launch: Option<String>,
    pub mission: Option<String>,
    pub reuse_count: Option<i32>,
    pub rtls_attempts: Option<i32>,
    pub rtls_landings: Option<i32>,
    pub asds_attempts: Option<i32>,
    pub asds_landings: Option<i32>,
    pub water_landing: Option<i32>,
}
