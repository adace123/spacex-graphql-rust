use super::{build_querystring, BaseQueryOptions};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct CapsuleQueryOptions {
    pub capsule_serial: Option<String>,
    pub capsule_id: Option<String>,
    pub status: Option<String>,
    pub original_launch: Option<String>,
    pub mission: Option<String>,
    pub landings: Option<i32>,

    #[serde(rename = "type")]
    #[graphql(name = "type")]
    pub _type: Option<String>,
}

impl BaseQueryOptions for CapsuleQueryOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("capsule_serial", self.capsule_serial),
            ("capsule_id", self.capsule_id),
            ("status", self.status),
            ("original_launch", self.original_launch),
            ("mission", self.mission),
            ("landings", self.landings.map(|l| l.to_string())),
            ("type", self._type),
        ];
        build_querystring(fields)
    }
}
