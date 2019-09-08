use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

pub trait SpaceXResource {}

#[derive(Serialize, Deserialize, Debug, GraphQLEnum)]
pub enum DateFilter {
    #[graphql(name = "upcoming")]
    Upcoming,
    #[graphql(name = "past")]
    Past,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl SpaceXResource for Location {}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Launchpad {
    pub id: i32,
    pub status: String,
    pub location: Location,
    pub vehicles_launched: Vec<String>,
    pub attempted_launches: i32,
    pub successful_launches: i32,
    pub wikipedia: String,
    pub details: String,
    pub site_id: String,
    pub site_name_long: String,
}

impl SpaceXResource for Launchpad {}
