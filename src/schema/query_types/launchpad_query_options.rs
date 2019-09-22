use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct LaunchpadQueryOptions {
    pub site_id: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
