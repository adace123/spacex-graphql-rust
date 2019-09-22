use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct ShipPosition {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct ShipMission {
    pub name: String,
    pub flight: i32
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Ship {
    pub ship_id: String,
    pub ship_name: String,
    pub ship_model: Option<String>,
    pub roles: Vec<String>,
    pub active: bool,
    pub imo: Option<i32>,
    pub mmsi: Option<i32>,
    pub abs: Option<i32>,
    pub class: Option<i32>,
    pub weight_lbs: Option<i32>,
    pub weight_kg: Option<i32>,
    pub year_built: Option<i32>,
    pub home_port: String,
    pub status: Option<String>,
    pub speed_kn: Option<f64>,
    pub course_deg: Option<i32>,
    pub position: ShipPosition,
    pub successful_landings: Option<i32>,
    pub attempted_landings: Option<i32>,
    pub missions: Vec<ShipMission>,
    pub url: Option<String>,
    pub image: Option<String>
}

impl SpaceXResource for Ship {
    fn resource_name() -> String {
        String::from("ships")
    }
}