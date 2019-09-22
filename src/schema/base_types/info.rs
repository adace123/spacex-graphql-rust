use crate::common::SpaceXResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Headquarters {
    pub address: String,
    pub city: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpaceXInfo {
    pub name: String,
    pub founder: String,
    pub founded: i32,
    pub employees: i32,
    pub vehicles: i32,
    pub launch_sites: i32,
    pub test_sites: i32,
    pub ceo: String,
    pub cto: String,
    pub coo: String,
    pub valuation: i64,
    pub cto_propulsion: String,
    pub headquarters: Headquarters,
    pub summary: String,
}

#[juniper::object]
impl Headquarters {
    fn address(&self) -> &str {
        self.address.as_str()
    }

    fn city(&self) -> &str {
        self.city.as_str()
    }

    fn state(&self) -> &str {
        self.state.as_str()
    }
}

#[juniper::object]
impl SpaceXInfo {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn founder(&self) -> &str {
        self.founder.as_str()
    }

    fn founded(&self) -> i32 {
        self.founded
    }

    fn employees(&self) -> i32 {
        self.employees
    }

    fn vehicles(&self) -> i32 {
        self.vehicles
    }

    fn launch_sites(&self) -> i32 {
        self.launch_sites
    }

    fn test_sites(&self) -> i32 {
        self.test_sites
    }

    fn ceo(&self) -> &str {
        self.ceo.as_str()
    }

    fn cto(&self) -> &str {
        self.cto.as_str()
    }

    fn coo(&self) -> &str {
        self.coo.as_str()
    }

    fn valuation(&self) -> String {
        self.valuation.to_string()
    }

    fn cto_propulsion(&self) -> &str {
        self.cto_propulsion.as_str()
    }

    fn headquarters(&self) -> &Headquarters {
        &self.headquarters
    }

    fn summary(&self) -> &str {
        self.summary.as_str()
    }
}

impl SpaceXResource for SpaceXInfo {
    fn resource_name() -> String {
        String::from("info")
    }
}
