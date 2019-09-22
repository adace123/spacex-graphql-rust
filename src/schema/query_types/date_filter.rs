use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, GraphQLEnum)]
pub enum DateFilter {
    #[graphql(name = "upcoming")]
    Upcoming,
    #[graphql(name = "past")]
    Past,
}

impl std::fmt::Display for DateFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt_string = match *self {
            DateFilter::Upcoming => "upcoming",
            DateFilter::Past => "past",
        };
        write!(f, "/{}/", fmt_string)
    }
}
