use super::{build_querystring, BaseQueryOptions, Ordering};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct OutputControlOptions {
    id: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
    sort: Option<String>,
    order: Option<Ordering>,
}

impl BaseQueryOptions for OutputControlOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("id", self.id.map(|i| i.to_string())),
            ("limit", self.limit.map(|l| l.to_string())),
            ("offset", self.offset.map(|o| o.to_string())),
            ("sort", self.sort.map(|s| s.to_string())),
            ("order", self.order.map(|o| o.to_string())),
        ];
        build_querystring(fields)
    }
}
