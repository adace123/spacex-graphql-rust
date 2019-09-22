use crate::common::SpaceXResource;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct HistoryLinks {
    pub reddit: Option<String>,
    pub article: String,
    pub wikipedia: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct HistoricalEvent {
    pub id: i32,
    pub title: String,
    pub event_date_utc: String,
    pub event_date_unix: i32,
    pub flight_number: Option<i32>,
    pub details: String,
    pub links: HistoryLinks,
}

impl SpaceXResource for HistoricalEvent {
    fn resource_name() -> String {
        String::from("history")
    }
}
