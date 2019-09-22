use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketHeight {
    pub meters: Option<f64>,
    pub feet: Option<f64>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketDiameter {
    pub meters: Option<f64>,
    pub feet: Option<f64>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketMass {
    pub kg: i32,
    pub lb: i32
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct PayloadWeight {
    pub id: String,
    pub name: String,
    pub kg: i32,
    pub lb: i32
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketThrust {
    pub kN: i32,
    pub lbf: i32
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketVacuum {
    pub kN: i32,
    pub lbf: i32
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct CompositeFairing {
    pub height: RocketHeight,
    pub diameter: RocketDiameter
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketPayload {
    option_1: String,
    option_2: Option<String>,
    composite_fairing: CompositeFairing
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketFirstStage {
    pub reusable: bool,
    pub engines: i32,
    pub fuel_amount_tons: f64,
    pub burn_time_sec: Option<i32>,
    pub thrust_sea_level: RocketThrust,
    pub thrust_vacuum: RocketVacuum
} 

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketEngine {
    pub number: i32,
    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: String,
    pub version: String,
    pub layout: Option<String>,
    pub engine_loss_max: Option<i32>,
    pub propellant_1: String,
    pub propellant_2: String, 
    pub thrust_sea_level: RocketThrust,
    pub thrust_vacuum: RocketVacuum,
    pub thrust_to_weight: Option<f64>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketLandingLegs {
    pub number: i32,
    pub material: Option<String>
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct RocketSecondStage {
    pub engines: i32,
    pub fuel_amount_tons: f64, 
    pub burn_time_sec: Option<i32>,
    pub thrust: RocketThrust ,
    pub payloads: RocketPayload
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Rocket {
    pub id: i32,
    pub active: bool,
    pub stages: i32,
    pub boosters: i32,
    pub cost_per_launch: i32,
    pub success_rate_pct: i32,
    pub first_flight: String,
    pub country: String,
    pub company: String,
    pub height: RocketHeight,
    pub diameter: RocketDiameter,
    pub mass: RocketMass,
    pub payload_weights: Vec<PayloadWeight>,
    pub first_stage: RocketFirstStage,
    pub second_stage: RocketSecondStage,
    pub engines: RocketEngine,
    pub landing_legs: RocketLandingLegs,
    pub wikipedia: String,
    pub description: String,
    pub rocket_id: String,
    pub rocket_name: String,
    pub rocket_type: String
}

impl SpaceXResource for Rocket {
    fn resource_name() -> String {
        String::from("rockets")
    }
}