use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct CoreQueryOptions {
    pub core_serial: Option<String>,
    pub block: Option<i32>,
    pub capsule_id: Option<String>,
    pub status: Option<String>,
    pub original_launch: Option<String>,
    pub mission: Option<String>,
    pub reuse_count: Option<i32>,
    pub rtls_attempts: Option<i32>,
    pub rtls_landings: Option<i32>,
    pub asds_attempts: Option<i32>,
    pub asds_landings: Option<i32>,
    pub water_landing: Option<i32>
}

impl BaseQueryOptions for CoreQueryOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("capsule_id", self.capsule_id),
            ("core_serial", self.core_serial),
            ("block", self.block.map(|b| b.to_string())),
            ("status", self.status),
            ("original_launch", self.original_launch),
            ("mission", self.mission),
            ("rtls_attempts", self.rtls_attempts.map(|r| r.to_string())),
            ("asds_attempts", self.asds_attempts.map(|a| a.to_string())),
            ("rtls_landings", self.rtls_landings.map(|l| l.to_string())),
            ("asds_landings", self.asds_landings.map(|a| a.to_string())),
            ("water_landing", self.water_landing.map(|w| w.to_string())),
            ("reuse_count", self.reuse_count.map(|l| l.to_string()))
        ];
        build_querystring(fields)
    }
}
