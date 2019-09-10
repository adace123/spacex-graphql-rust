use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

pub trait SpaceXResource {
    
}

#[derive(Serialize, Deserialize, Debug, GraphQLEnum)]
pub enum DateFilter {
    #[graphql(name = "upcoming")]
    Upcoming,
    #[graphql(name = "past")]
    Past,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchpadLocation {
    pub name: String,
    pub region: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl SpaceXResource for LaunchpadLocation {
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Launchpad {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub location: LaunchpadLocation,
    pub vehicles_launched: Vec<String>,
    pub attempted_launches: i32,
    pub successful_launches: i32,
    pub wikipedia: String,
    pub details: String,
    pub site_id: String,
    pub site_name_long: String,
}

impl SpaceXResource for Launchpad {}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Mission {
    pub name: String,
    pub flight: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Capsule {
    pub capsule_serial: String,
    pub capsule_id: String,
    pub status: String,
    pub original_launch: Option<String>,
    pub original_launch_unix: Option<i32>,
    pub missions: Vec<Mission>,
    pub landings: i32,

    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: String,
    pub details: Option<String>,
    pub reuse_count: i32,
}

impl SpaceXResource for Capsule {

}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Core {
    pub core_serial: String,
    pub block: i32,
    pub status: String,
    pub original_launch: Option<String>,
    pub original_launch_unix: Option<String>,
    pub missions: Vec<Mission>,
    pub reuse_count: i32,
    pub rtls_attempts: i32,
    pub rtls_landings: i32,
    pub asds_attempts: i32,
    pub asds_landings: i32,
    pub water_landing: bool,
    pub details: Option<String>
}

impl SpaceXResource for Core {

}