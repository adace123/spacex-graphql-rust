use juniper::{EmptyMutation, RootNode};

pub mod context;
pub use context::Context;

mod query_impl;
use query_impl::Query;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;
pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new())
}
