use super::{build_querystring, BaseQueryOptions, Ordering};
use juniper::GraphQLInputObject;
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct OutputControlOptions {
    id: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
    sort: Option<String>,
    order: Option<Ordering>,
}
