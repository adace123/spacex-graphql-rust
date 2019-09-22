use juniper::{FieldResult, FieldError, graphql_value};
use crate::base_types::*;
use crate::query_types::*;
use crate::common::*;
use super::context::Context;

pub struct Query;

fn get_resource_collection<T, U>(
    context: &Context,
    date_filter: Option<DateFilter>,
    query_options: Option<U>,
    control_options: Option<OutputControlOptions>,
) -> FieldResult<Vec<T>>
where
    T: SpaceXResource + serde::de::DeserializeOwned,
    U: BaseQueryOptions + serde::de::DeserializeOwned,
{
    let mut url: String = format!("{}/{}", context.base_url, T::resource_name());
    url.push_str(
        &date_filter
            .map(|d| d.to_string())
            .unwrap_or(String::from("")),
    );
    url.push_str(
        &control_options
            .map(|c| c.get_querystring())
            .unwrap_or(String::from("")),
    );
    url.push_str(
        &query_options
            .map(|q| q.get_querystring())
            .unwrap_or(String::from("")),
    );

    info!("Sending request to: {}", url);
    let collections: Vec<T> = reqwest::get(&url)?.json()?;
    Ok(collections)
}

fn get_single_resource<T, U>(context: &Context, id: U) -> FieldResult<T>
where
    T: SpaceXResource + serde::de::DeserializeOwned,
    U: std::fmt::Display,
{
    let mut url: String = format!("{}/{}", context.base_url, T::resource_name());
    let format_id = match !id.to_string().is_empty() {
        true => format!("/{}", id),
        false => String::from(""),
    };
    url.push_str(&format_id);
    let result: Result<T, reqwest::Error> = reqwest::get(&url)?.json();
    info!("Sending request to: {}", url);

    match result {
        Ok(l) => Ok(l),
        Err(e) => {
            error!("{:?}", e);

            Err(FieldError::new(
                format!("{:?}", e),
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

    fn cores(
        context: &Context,
        date_filter: Option<DateFilter>,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<CoreQueryOptions>,
    ) -> FieldResult<Vec<Core>> {
        get_resource_collection(context, date_filter, query_options, output_control_options)
    }

    fn core(context: &Context, id: String) -> FieldResult<Core> {
        get_single_resource(context, id)
    }

    fn dragons(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
    ) -> FieldResult<Vec<Dragon>> {
        get_resource_collection(
            context,
            None,
            None::<EmptyQueryOptions>,
            output_control_options,
        )
    }

    fn dragon(context: &Context, id: String) -> FieldResult<Dragon> {
        get_single_resource(context, id)
    }

    fn historical_events(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<HistoryQueryOptions>,
    ) -> FieldResult<Vec<HistoricalEvent>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn historical_event(context: &Context, id: i32) -> FieldResult<HistoricalEvent> {
        get_single_resource(context, id)
    }

    fn info(context: &Context) -> FieldResult<SpaceXInfo> {
        get_single_resource(context, "")
    }

    fn landingpads(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
    ) -> FieldResult<Vec<Landingpad>> {
        get_resource_collection(
            context,
            None,
            None::<EmptyQueryOptions>,
            output_control_options,
        )
    }

    fn landingpad(context: &Context, id: String) -> FieldResult<Landingpad> {
        get_single_resource(context, id)
    }
    // TODO: launches
    fn launches(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
    ) -> FieldResult<Vec<Launch>> {
        get_resource_collection(
            context,
            None,
            None::<EmptyQueryOptions>,
            output_control_options,
        )
    }

    fn launchpads(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<LaunchpadQueryOptions>,
    ) -> FieldResult<Vec<Launchpad>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn launchpad(context: &Context, site_id: String) -> FieldResult<Launchpad> {
        get_single_resource(context, site_id)
    }

}