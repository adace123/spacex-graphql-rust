pub mod types;
use juniper::{graphql_value, EmptyMutation, FieldError, FieldResult, RootNode};
use types::*;

pub struct Context {
    pub base_url: String,
}

impl juniper::Context for Context {}

pub struct Query;

fn get_resource_collection<T>(
    context: &Context,
    resource_name: String,
    limit: Option<i32>,
    offset: Option<i32>,
    date_filter: Option<DateFilter>,
) -> FieldResult<Vec<T>>
where
    T: SpaceXResource,
    T: serde::de::DeserializeOwned,
{
    let mut url: String = format!("{}/{}", context.base_url, resource_name);
    url.push_str(date_filter.map_or_else(
        || "",
        |filter| match filter {
            DateFilter::Past => "/past",
            DateFilter::Upcoming => "/upcoming",
        },
    ));
    url.push_str(
        &limit
            .map(|l| format!("?limit={}", l))
            .unwrap_or("".to_string()),
    );
    url.push_str(
        &offset
            .map(|l| format!("&offset={}", l))
            .unwrap_or("".to_string()),
    );
    let collections: Vec<T> = reqwest::get(&url)?.json()?;
    Ok(collections)
}

fn get_single_resource<T>(context: &Context, resource_name: String, id: String) -> FieldResult<T>
where
    T: SpaceXResource,
    T: serde::de::DeserializeOwned,
{
    let url: String = format!("{}/{}/{}", context.base_url, resource_name, id);
    let result: Result<T, reqwest::Error> = reqwest::get(&url)?.json();
    match result {
        Ok(l) => Ok(l),
        Err(_) => {
            let error_message: String = format!("{} with ID {} not found", resource_name, id);
            Err(FieldError::new(
                error_message.as_str(),
                graphql_value!({
                    "error_type": "not_found"
                }),
            ))
        }
    }
}

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn launchpads(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> FieldResult<Vec<Launchpad>> {
        get_resource_collection(context, "launchpads".to_owned(), limit, offset, None)
    }

    fn launchpad(context: &Context, id: String) -> FieldResult<Launchpad> {
        get_single_resource(context, "launchpads".to_owned(), id)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;
pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new())
}
