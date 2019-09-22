use super::{build_querystring, BaseQueryOptions};
use juniper::GraphQLInputObject;
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct CapsuleQueryOptions {
    pub capsule_serial: Option<String>,
    pub capsule_id: Option<String>,
    pub status: Option<String>,
    pub original_launch: Option<String>,
    pub mission: Option<String>,
    pub landings: Option<i32>,

    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: Option<String>,
}
