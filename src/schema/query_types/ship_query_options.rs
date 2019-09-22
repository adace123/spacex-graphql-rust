use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct ShipQueryOptions {
    pub ship_id: Option<String>,
    pub ship_name: Option<String>,
    pub ship_model: Option<String>,
    pub ship_type: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
    pub imo: Option<i32>,
    pub mmsi: Option<i32>,
    pub abs: Option<i32>,
    pub class: Option<i32>,
    pub weight_lbs: Option<i32>,
    pub weight_kg: Option<i32>,
    pub year_built: Option<i32>,
    pub home_port: Option<String>,
    pub status: Option<String>,
    pub speed_kn: Option<i32>,
    pub course_deg: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub successful_landings: Option<i32>,
    pub attempted_landings: Option<i32>,
    pub mission: Option<String>
}