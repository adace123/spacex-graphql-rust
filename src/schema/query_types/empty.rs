use super::{build_querystring, BaseQueryOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyQueryOptions {}

impl BaseQueryOptions for EmptyQueryOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![];
        build_querystring(fields)
    }
}
