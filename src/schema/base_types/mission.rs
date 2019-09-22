use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Mission {
    pub mission_name: String,
    pub mission_id: String,
    pub manufacturers: Vec<String>,
    pub payload_ids: Vec<String>,
    pub wikipedia: String,
    pub website: String,
    pub twitter: Option<String>,
    pub description: String,
}

impl SpaceXResource for Mission {
    fn resource_name() -> String {
        String::from("missions")
    }
}
