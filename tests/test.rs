#[cfg(test)]
mod tests {
    use spacex_api_rust_graphql::api::query_types::*;
    use spacex_api_rust_graphql::api::base_types::*;
    use reqwest;

    #[test]
    fn test_request() {
        let base_url = "https://api.spacexdata.com/v3";
        let query_options = LaunchpadQueryOptions {
            id: Some(true),
            limit: Some(3),
            offset: None
        };

        let querystring = query_options.get_querystring();
        assert_eq!(querystring, "?id=true&limit=3");
        let test_url = format!("{}/launchpads/{}", base_url, querystring);
        let mut res = reqwest::get(&test_url);
        println!("url is {}", test_url);
        assert!(res.is_ok());
        
        let json_vec: Result<Vec<Launchpad>, reqwest::Error> = res.unwrap().json();
        assert!(json_vec.is_ok());
        let json_vec: Vec<Launchpad> = json_vec.unwrap();
        assert_eq!(json_vec.len(), 3);

        let test_id = "vafb_slc_4e";
        res = reqwest::get(&format!("{}/launchpads/{}", base_url, test_id));
        assert!(res.is_ok());
        
        let json_result: Result<Launchpad, reqwest::Error> = res.unwrap().json();
        assert!(json_result.is_ok());

        let test_launchpad: Launchpad = json_result.unwrap();
        assert_eq!(test_launchpad.name, "VAFB SLC 4E");
        assert_eq!(test_launchpad.id, 6);
        assert_eq!(test_launchpad.location.name, "Vandenberg Air Force Base");
    }
}

