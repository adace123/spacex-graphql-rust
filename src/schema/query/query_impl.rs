use super::context::Context;
use crate::base_types::*;
use crate::common::*;
use crate::query_types::*;
use juniper::{graphql_value, FieldError, FieldResult};

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

fn get_single_resource<T, U>(
    context: &Context,
    id: Option<U>,
    launch_time_filter: Option<LaunchTimeFilter>,
) -> FieldResult<T>
where
    T: SpaceXResource + serde::de::DeserializeOwned,
    U: std::fmt::Display,
{
    let mut url: String = format!("{}/{}", context.base_url, T::resource_name());

    if let Some(i) = id {
        let format_id = match !i.to_string().is_empty() {
            true => format!("/{}", i),
            false => String::from(""),
        };
        url.push_str(&format_id);
    }

    if let Some(ltf) = launch_time_filter {
        url = format!("{}/{}", url, ltf);
    }

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

    fn capsule(context: &Context, id: Option<String>) -> FieldResult<Capsule> {
        get_single_resource(context, id, None)
    }

    fn cores(
        context: &Context,
        date_filter: Option<DateFilter>,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<CoreQueryOptions>,
    ) -> FieldResult<Vec<Core>> {
        get_resource_collection(context, date_filter, query_options, output_control_options)
    }

    fn core(context: &Context, id: Option<String>) -> FieldResult<Core> {
        get_single_resource(context, id, None)
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

    fn dragon(context: &Context, id: Option<String>) -> FieldResult<Dragon> {
        get_single_resource(context, id, None)
    }

    fn historical_events(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<HistoryQueryOptions>,
    ) -> FieldResult<Vec<HistoricalEvent>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn historical_event(context: &Context, id: Option<i32>) -> FieldResult<HistoricalEvent> {
        get_single_resource(context, id, None)
    }

    fn info(context: &Context) -> FieldResult<SpaceXInfo> {
        get_single_resource(context, Some(""), None)
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

    fn landingpad(context: &Context, id: Option<String>) -> FieldResult<Landingpad> {
        get_single_resource(context, id, None)
    }

    fn launches(
        context: &Context,
        date_filter: Option<DateFilter>,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<LaunchQueryOptions>,
    ) -> FieldResult<Vec<Launch>> {
        get_resource_collection(context, date_filter, query_options, output_control_options)
    }

    fn launch(
        context: &Context,
        flight_number: Option<i32>,
        launch_time_filter: Option<LaunchTimeFilter>,
    ) -> FieldResult<Launch> {
        get_single_resource(context, flight_number, launch_time_filter)
    }

    fn launchpads(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<LaunchpadQueryOptions>,
    ) -> FieldResult<Vec<Launchpad>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn launchpad(context: &Context, site_id: Option<String>) -> FieldResult<Launchpad> {
        get_single_resource(context, site_id, None)
    }

    fn missions(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
    ) -> FieldResult<Vec<Mission>> {
        get_resource_collection(
            context,
            None,
            None::<EmptyQueryOptions>,
            output_control_options,
        )
    }

    fn mission(context: &Context, mission_id: Option<String>) -> FieldResult<Mission> {
        get_single_resource(context, mission_id, None)
    }

    fn payloads(
        context: &Context,
        output_control_options: Option<OutputControlOptions>,
        query_options: Option<PayloadQueryOptions>,
    ) -> FieldResult<Vec<LaunchPayload>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn payload(context: &Context, payload_id: Option<String>) -> FieldResult<LaunchPayload> {
        get_single_resource(context, payload_id, None)
    }

    fn rockets(context: &Context, output_control_options: Option<OutputControlOptions>) -> FieldResult<Vec<Rocket>> {
        get_resource_collection(context, None, None::<EmptyQueryOptions>, output_control_options)
    }

    fn rocket(context: &Context, rocket_id: Option<String>) -> FieldResult<Rocket> {
        get_single_resource(context, rocket_id, None)
    }

    fn roadster(context: &Context) -> FieldResult<TeslaRoadster> {
        get_single_resource(context, None::<TeslaRoadster>, None)
    }

    fn ships(context: &Context, output_control_options: Option<OutputControlOptions>, query_options: Option<ShipQueryOptions>) -> FieldResult<Vec<Ship>> {
        get_resource_collection(context, None, query_options, output_control_options)
    }

    fn ship(context: &Context, ship_id: Option<String>) -> FieldResult<Ship> {
        get_single_resource(context, ship_id, None)
    }
}
