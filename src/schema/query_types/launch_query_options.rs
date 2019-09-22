use super::{build_querystring, BaseQueryOptions};
pub use juniper::{GraphQLEnum, GraphQLInputObject};
use querystring_macro::query_string_builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLEnum)]
pub enum LaunchTimeFilter {
    Latest,
    Next,
}

impl std::fmt::Display for LaunchTimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt_string = match *self {
            LaunchTimeFilter::Latest => "latest",
            LaunchTimeFilter::Next => "next",
        };
        write!(f, "{}/", fmt_string)
    }
}

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject, query_string_builder)]
pub struct LaunchQueryOptions {
    pub flight_id: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub flight_number: Option<i32>,
    pub launch_year: Option<i32>,
    pub launch_date_utc: Option<i32>,
    pub launch_date_local: Option<i32>,
    pub tbd: Option<bool>,
    pub rocket_id: Option<String>,
    pub rocket_name: Option<String>,
    pub rocket_type: Option<String>,
    pub core_serial: Option<String>,
    pub land_success: Option<bool>,
    pub landing_intent: Option<bool>,
    pub landing_type: Option<String>,
    pub landing_vehicle: Option<String>,
    pub cap_serial: Option<String>,
    pub core_flight: Option<i32>,
    pub block: Option<i32>,
    pub gridfins: Option<bool>,
    pub legs: Option<bool>,
    pub core_reuse: Option<bool>,
    pub capsule_reuse: Option<bool>,
    pub fairings_reused: Option<bool>,
    pub fairings_recovery_attempt: Option<bool>,
    pub fairings_recovered: Option<bool>,
    pub fairings_ship: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub payload_id: Option<String>,
    pub norad_id: Option<i32>,
    pub customer: Option<String>,
    pub nationality: Option<String>,
    pub manufacturer: Option<String>,
    pub payload_type: Option<String>,
    pub orbit: Option<String>,
    pub reference_system: Option<String>,
    pub regime: Option<String>,
    pub longitude: Option<f64>,
    pub semi_major_axis_km: Option<f64>,
    pub eccentricity: Option<f64>,
    pub periapsis_km: Option<f64>,
    pub apoapsis_km: Option<f64>,
    pub inclination_deg: Option<f64>,
    pub period_min: Option<f64>,
    pub lifespan_years: Option<i32>,
    pub epoch: Option<String>,
    pub mean_motion: Option<f64>,
    pub raan: Option<f64>,
    pub launch_success: Option<bool>,
}
