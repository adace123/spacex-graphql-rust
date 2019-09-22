use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct OrbitParams {
    pub reference_system: Option<String>,
    pub regime: Option<String>,
    pub longitude: Option<f64>,
    pub lifespan_years: Option<f64>,
    pub epoch: Option<String>,
    pub semi_major_axis_km: Option<f64>,
    pub eccentricity: Option<f64>,
    pub mean_motion: Option<f64>,
    pub raan: Option<f64>,
    pub arg_of_pericenter: Option<f64>,
    pub mean_anomaly: Option<f64>,
    pub inclination_deg: Option<f64>,
    pub period_min: Option<f64>,
    pub periapsis_km: Option<f64>,
    pub apoapsis_km: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchPayload {
    pub payload_id: Option<String>,
    pub norad_id: Option<Vec<i32>>,
    pub reused: bool,
    pub customers: Option<Vec<String>>,
    pub nationality: Option<String>,
    pub manufacturer: Option<String>,
    pub payload_type: Option<String>,
    pub payload_mass_kg: Option<f64>,
    pub payload_mass_lbs: Option<f64>,
    pub orbit: Option<String>,
    pub orbit_params: OrbitParams,
}

impl SpaceXResource for LaunchPayload {
    fn resource_name() -> String {
        String::from("payloads")
    }
}
