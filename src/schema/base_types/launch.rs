use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchCore {
    pub core_serial: Option<String>,
    pub flight: Option<i32>,
    pub block: Option<i32>,
    pub gridfins: Option<bool>,
    pub legs: Option<bool>,
    pub reused: Option<bool>,
    pub land_success: Option<bool>,
    pub landing_intent: Option<bool>,
    pub landing_type: Option<String>,
    pub landing_vehicle: Option<String>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct FirstStage {
    pub cores: Vec<LaunchCore>
}

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
    pub apoapsis_km: Option<f64>
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
    pub orbit_params: OrbitParams
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct SecondStage {
    pub block: Option<i32>,
    pub payloads: Vec<LaunchPayload>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Fairings {
    pub reused: bool,
    pub recovery_attempt: Option<bool>,
    pub recovered: Option<bool>,
    pub ship: Option<String>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchRocket {
    pub rocket_id: Option<String>,
    pub rocket_name: Option<String>,
    pub rocket_type: Option<String>,
    pub first_stage: FirstStage,
    pub second_stage: SecondStage,
    pub fairings: Option<Fairings>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchTelemetry {
    pub flight_club: Option<String>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchSite {
    site_id: Option<String>,
    site_name: Option<String>,
    site_name_long: Option<String>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchLinks {
    pub mission_patch: Option<String>,
    pub mission_patch_small: Option<String>,
    pub reddit_campaign: Option<String>,
    pub reddit_launch: Option<String>,
    pub reddit_recovery: Option<String>,
    pub reddit_media: Option<String>,
    pub presskit: Option<String>,
    pub article_link: Option<String>,
    pub wikipedia: Option<String>,
    pub video_link: Option<String>,
    pub youtube_id: Option<String>,
    pub flickr_images: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchTimeline {
    pub webcast_liftoff: Option<i32>,
    pub go_for_prop_loading: Option<i32>,
    pub rp1_loading: Option<i32>,
    pub stage1_lox_loading: Option<i32>,
    pub stage2_lox_loading: Option<i32>,
    pub engine_chill: Option<i32>,
    pub prelaunch_checks: Option<i32>,
    pub propellant_pressurization: Option<i32>,
    pub go_for_launch: Option<i32>,
    pub ignition: Option<i32>,
    pub liftoff: Option<i32>,
    pub maxq: Option<i32>,
    pub stage_sep: Option<i32>,
    pub second_stage_ignition: Option<i32>,
    pub fairing_deploy: Option<i32>,
    pub first_stage_entry_burn: Option<i32>,
    pub first_stage_landing: Option<i32>,
    pub seco_1: Option<i32>,
    pub second_stage_restart: Option<i32>,
    pub seco_2: Option<i32>,
    pub payload_deploy: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Launch {
    pub flight_number: i32,
    pub mission_name: String,
    pub mission_id: Vec<String>,
    pub launch_year: String,
    pub launch_date_unix: i32,
    pub launch_date_utc: Option<String>,
    pub launch_window: Option<i32>,
    pub is_tentative: bool,
    pub tentative_max_precision: Option<String>,
    pub tbd: bool,
    pub rocket: LaunchRocket,
    pub ships: Vec<String>,
    pub telemetry: LaunchTelemetry,
    pub launch_site: LaunchSite,
    pub launch_success: Option<bool>,
    pub links: LaunchLinks,
    pub details: Option<String>,
    pub upcoming: bool,
    pub static_fire_date_utc: Option<String>,
    pub static_fire_date_unix: Option<i32>,
    pub timeline: Option<LaunchTimeline>,
    pub crew: Option<Vec<String>>
}

impl SpaceXResource for Launch {
    fn resource_name() -> String {
        String::from("launches")
    }
}