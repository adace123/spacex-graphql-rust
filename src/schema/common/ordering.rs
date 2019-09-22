use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLEnum)]
pub enum Ordering {
    Asc,
    Desc,
}

impl std::fmt::Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt_string = match *self {
            Ordering::Asc => "asc",
            Ordering::Desc => "desc",
        };
        write!(f, "{}", fmt_string)
    }
}
