pub mod base_types;
pub mod query_types;
use juniper::{graphql_value, EmptyMutation, FieldError, FieldResult, RootNode};
use base_types::*;
use query_types::*;

extern crate pretty_env_logger;

pub struct Context {
    pub base_url: String,
}

impl juniper::Context for Context {}

pub struct Query;

fn get_resource_collection<T, U>(
    context: &Context,
    date_filter: Option<DateFilter>,
    query_options: Option<U>,
    control_options: Option<OutputControlOptions>
) -> FieldResult<Vec<T>>
where
    T: SpaceXResource + serde::de::DeserializeOwned,
    U: BaseQueryOptions + serde::de::DeserializeOwned
{
    let mut url: String = format!("{}/{}", context.base_url, T::resource_name());
    url.push_str(&date_filter.map(|d| d.to_string()).unwrap_or(String::from("")));
    url.push_str(&control_options.map(|c| c.get_querystring()).unwrap_or(String::from("")));
    url.push_str(&query_options.map(|q| q.get_querystring()).unwrap_or(String::from("")));

    info!("Sending request to: {}", url);
    let collections: Vec<T> = reqwest::get(&url)?.json()?;
    Ok(collections)
}

fn get_single_resource<T>(context: &Context, id: String) -> FieldResult<T>
where
    T: SpaceXResource,
    T: serde::de::DeserializeOwned,
{
    let url: String = format!("{}/{}/{}", context.base_url, T::resource_name(), id);
    let result: Result<T, reqwest::Error> = reqwest::get(&url)?.json();
    info!("Sending request to: {}", url);

    match result {
        Ok(l) => Ok(l),
        Err(_) => {
            let error_message: String = format!("{} with ID {} not found", T::resource_name(), id);
            error!("{}", error_message);

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
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<LaunchpadQueryOptions>
    ) -> FieldResult<Vec<Launchpad>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn launchpad(context: &Context, id: String) -> FieldResult<Launchpad> {
        get_single_resource(context, id)
    }

    fn capsules(
        context: &Context,
        date_filter: Option<DateFilter>,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<CapsuleQueryOptions>,
    ) -> FieldResult<Vec<Capsule>> {
        get_resource_collection(context, date_filter, query_options, output_control_options)
    }

    fn capsule(context: &Context, id: String) -> FieldResult<Capsule> {
        get_single_resource(context, id)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;
pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new())
}
