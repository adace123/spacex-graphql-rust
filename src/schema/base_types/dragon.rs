use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct HeatShield {
    pub material: String,
    pub size_meters: f64,
    pub temp_degrees: i32,
    pub dev_partner: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Thrust {
    pub kN: f64,
    pub lbf: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Thruster {
    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: String,
    pub amount: i32,
    pub pods: i32,
    pub fuel_1: String,
    pub fuel_2: String,
    pub thrust: Thrust,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct LaunchPayloadMass {
    pub kg: i32,
    pub lb: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Volume {
    pub cubic_meters: i32,
    pub cubic_feet: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct MetersFeet {
    pub meters: f64,
    pub feet: f64,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Mass {
    pub kg: i32,
    pub lb: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct PressurizedCapsule {
    pub payload_volume: Volume,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Cargo {
    pub solar_array: i32,
    pub unpressurized_cargo: bool,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Trunk {
    pub trunk_volume: Volume,
    pub cargo: Cargo,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct Dragon {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: String,
    pub active: bool,
    pub crew_capacity: i32,
    pub sidewall_angle_deg: i32,
    pub orbit_duration_yr: i32,
    pub dry_mass_kg: i32,
    pub dry_mass_lb: i32,
    pub first_flight: String,
    pub heat_shield: HeatShield,
    pub thrusters: Vec<Thruster>,
    pub launch_payload_mass: Mass,
    pub launch_payload_vol: Volume,
    pub return_payload_mass: Mass,
    pub return_payload_vol: Volume,
    pub pressurized_capsule: PressurizedCapsule,
    pub trunk: Trunk,
    pub height_w_trunk: MetersFeet,
    pub diameter: MetersFeet,
    pub flickr_images: Vec<String>,
    pub wikipedia: String,
    pub description: String,
}

impl SpaceXResource for Dragon {
    fn resource_name() -> String {
        String::from("dragons")
    }
}
