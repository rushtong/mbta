use config::Config;

mod config;
mod routes;

fn main() -> () {
    let config: Config = config::parse_config("./config.yaml").unwrap();
    let api_key: String = config.api_key;

    println!("Listing Routes");
    let routes = routes::query_routes(true, Some(api_key.clone()));
    routes.unwrap().data.iter().for_each(|data| {
        println!("{}", data)
    });
    
    println!("Listing Stops for Red Line");
    let red_line = routes::query_route_stops(false, Some(api_key.clone()), "Red".to_string());
    red_line.unwrap().data.iter().for_each(|data| {
        println!("{}", data)
    });

    println!("Listing Stops for Orange Line");
    let orange_line = routes::query_route_stops(false, Some(api_key.clone()), "Orange".to_string());
    orange_line.unwrap().data.iter().for_each(|data| {
        println!("{}", data)
    });

    println!("Listing Stops for all Green Lines");
    let green_line = routes::query_route_stops(false, Some(api_key.clone()), "Green-B,Green-C,Green-D,Green-E".to_string());
    green_line.unwrap().data.iter().for_each(|data| {
        println!("{}", data)
    });

}
