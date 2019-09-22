use crate::common::{SpaceXResource};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchpadLocation {
    pub name: String,
    pub region: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl SpaceXResource for LaunchpadLocation {
    fn resource_name() -> String {
        String::from("launchpad_location")
    }
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

impl SpaceXResource for Launchpad {
    fn resource_name() -> String {
        String::from("launchpads")
    }
}
