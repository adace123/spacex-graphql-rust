pub trait SpaceXResource {
    fn resource_name() -> String;
}

pub trait BaseQueryOptions {
    fn get_querystring(self) -> String;
}

pub mod ordering;
pub use ordering::*;
