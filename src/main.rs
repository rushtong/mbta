use config::Config;

mod config;
mod routes;

fn main() -> () {
    let config: Config = config::parse_config("./config.yaml").unwrap();
    let api_key: String = config.api_key;

    let routes = routes::query_routes(false, Some(api_key.clone()));
    routes.unwrap().data.iter().for_each(|data| {
        let long_name = &data.attributes.long_name;
        let id = &data.id;
        let dashes = std::iter::repeat("-").take(long_name.chars().count()).collect::<String>();
        println!("-------------------{}", dashes);
        println!("Listing Stops for: {}", long_name);
        println!("-------------------{}", dashes);
        let line = routes::query_route_stops(false, Some(api_key.clone()), id.clone());
        line.unwrap().data.iter().for_each(|data| {
            println!("{}", data)
        });
    });

}
