use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct CoreMission {
    pub name: String,
    pub flight: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Core {
    pub core_serial: String,
    pub block: Option<i32>,
    pub status: String,
    pub original_launch: Option<String>,
    pub original_launch_unix: Option<i32>,
    pub missions: Vec<CoreMission>,
    pub reuse_count: i32,
    pub rtls_attempts: i32,
    pub rtls_landings: i32,
    pub asds_attempts: i32,
    pub asds_landings: i32,
    pub water_landing: bool,
    pub details: Option<String>,
}

impl SpaceXResource for Core {
    fn resource_name() -> String {
        String::from("cores")
    }
}
