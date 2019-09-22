use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct TeslaRoadster {
    pub name: String,
    pub launch_date_utc: String,
    pub launch_date_unix: i32,
    pub launch_mass_kg: i32,
    pub launch_mass_lbs: i32,
    pub norad_id: i32,
    pub epoch_jd: f64,
    pub orbit_type: String,
    pub apoapsis_au: f64,
    pub periapsis_au: f64,
    pub semi_major_axis_au: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub longitude: f64,
    pub periapsis_arg: f64,
    pub period_days: f64,
    pub speed_kph: f64,
    pub speed_mph: f64,
    pub earth_distance_km: f64,
    pub earth_distance_mi: f64,
    pub mars_distance_km: f64,
    pub mars_distance_mi: f64,
    pub wikipedia: String,
    pub details: String
}

impl SpaceXResource for TeslaRoadster {
    fn resource_name() -> String {
        String::from("roadster")
    }
}

impl std::fmt::Display for TeslaRoadster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: Exploring the final frontier...")
    }
}