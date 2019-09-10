use juniper::{GraphQLInputObject, GraphQLEnum};
use serde::{Deserialize, Serialize};

pub fn build_querystring(options: Vec<(&str, Option<String>)>) -> String {
    match options.is_empty() {
        true => String::from(""),
        false => {
            let querystring: String = options.iter()
                .filter(|(_, value)| value.is_some())
                .map(|(key, value)| format!("{}={}", key, value.as_ref().unwrap()))
                .collect::<Vec<String>>()
                .join("&");
            format!("?{}", querystring)
        }
    }
}

pub trait BaseQueryOptions {
    fn get_querystring(self) -> String;
}

#[derive(Serialize, Deserialize, Debug, GraphQLEnum)]
pub enum Ordering {
    Asc, 
    Desc
}

impl std::fmt::Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt_string = match *self {
            Ordering::Asc => "asc",
            Ordering::Desc => "desc"
        };
        write!(f, "{}", fmt_string)
    }
}

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct OutputControlOptions {
    id: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
    sort: Option<String>,
    order: Option<Ordering>
}


impl BaseQueryOptions for OutputControlOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec! [
            ("id", self.id.map(|i| i.to_string())),
            ("limit", self.limit.map(|l| l.to_string())),
            ("offset", self.offset.map(|o| o.to_string())),
            ("sort", self.sort.map(|s| s.to_string())),
            ("order", self.order.map(|o| o.to_string()))
        ];
        build_querystring(fields)
    }
}

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct CapsuleQueryOptions {
    pub capsule_serial: Option<String>,
    pub capsule_id: Option<String>,
    pub status: Option<String>,
    pub original_launch: Option<String>,
    pub mission: Option<String>,
    pub landings: Option<i32>,

    #[serde(rename = "type")]
    #[graphql(name="type")]
    pub _type: Option<String>
}

impl BaseQueryOptions for CapsuleQueryOptions {
    fn get_querystring(self) -> String 
    {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("capsule_serial", self.capsule_serial),
            ("capsule_id", self.capsule_id),
            ("status", self.status),
            ("original_launch", self.original_launch),
            ("mission", self.mission),
            ("landings", self.landings.map(|l| l.to_string())),
            ("type", self._type)
        ];
        build_querystring(fields)
    }
}

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct LaunchpadQueryOptions {
    pub id: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>
}

impl BaseQueryOptions for LaunchpadQueryOptions {
    fn get_querystring(self) -> String {
        let fields: Vec<(&str, Option<String>)> = vec![
            ("id", self.id.map(|i| i.to_string())),
            ("limit", self.limit.map(|l| l.to_string())),
            ("offset", self.offset.map(|o| o.to_string()))
        ];
        build_querystring(fields)
    }
}
