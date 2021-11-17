use reqwest::blocking::{Client, Response};
use reqwest::Error;
use reqwest::header::ACCEPT;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use route_models::Routes;
use stop_models::Stops;

mod route_models;
mod stop_models;

pub fn query_routes(debug: bool, api_key: Option<String>) -> Result<Routes, Error> {
    // The light and heavy rail routes are types 0 and 1
    let request_url: &str = "https://api-v3.mbta.com/routes?filter[type]=0,1";
    let response: Response = query_endpoint(request_url, api_key).unwrap();
    log_debug(debug, request_url.to_string(), response.status().clone(), response.headers().clone());
    let routes: route_models::Routes = response.json()?;
    Ok(routes)
}

pub fn query_route_stops(debug: bool, api_key: Option<String>, route_id: String) -> Result<Stops, Error> {
    let request_url: String = format!("https://api-v3.mbta.com/stops?filter[route]={}", route_id);
    let response: Response = query_endpoint(request_url.as_str(), api_key).unwrap();
    log_debug(debug, request_url, response.status().clone(), response.headers().clone());
    let stops: stop_models::Stops = response.json()?;
    Ok(stops)
}

fn query_endpoint(url: &str, api_key: Option<String>) -> Result<Response, Error> {
    let client: Client = reqwest::blocking::Client::new();
    let response: Response = match api_key {
        Some(key) => client.get(url)
            .header(ACCEPT, "application/vnd.api+json")
            .header("X-API-Key", key)
            .send()
            .unwrap(),
        None => client.get(url)
            .header(ACCEPT, "application/vnd.api+json")
            .send()
            .unwrap()
    };
    Ok(response)
}

fn log_debug(debug: bool, request_url: String, status: StatusCode, headers: HeaderMap) {
    if debug {
        println!("Request URL: {}", request_url);
        println!("Status: {}", status);
        headers.iter().for_each(|(n, v)| {
            println!("Header: {}:{}", n, v.to_str().unwrap());
        });
    }
}