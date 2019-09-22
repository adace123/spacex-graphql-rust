use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LandingpadLocation {
    pub name: String,
    pub region: String,
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Landingpad {
    pub id: String,
    pub full_name: String,
    pub location: LandingpadLocation,
    pub landing_type: String,
    pub attempted_landings: i32,
    pub successful_landings: i32,
    pub wikipedia: String,
    pub details: String
}

impl SpaceXResource for Landingpad {
    fn resource_name() -> String {
        String::from("landpads")
    }
}
