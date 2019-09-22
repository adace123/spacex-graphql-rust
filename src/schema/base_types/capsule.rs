use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct CapsuleMission {
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
    pub missions: Vec<CapsuleMission>,
    pub landings: i32,

    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: String,
    pub details: Option<String>,
    pub reuse_count: i32,
}

impl SpaceXResource for Capsule {
    fn resource_name() -> String {
        String::from("capsules")
    }
}
