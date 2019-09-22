use super::{build_querystring, BaseQueryOptions};
pub use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct LaunchpadQueryOptions {
    pub site_id: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl BaseQueryOptions for LaunchpadQueryOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("site_id", self.site_id.map(|i| i.to_string())),
            ("limit", self.limit.map(|l| l.to_string())),
            ("offset", self.offset.map(|o| o.to_string())),
        ];
        build_querystring(fields)
    }
}
