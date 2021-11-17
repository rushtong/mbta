use std::fmt;

use serde::Deserialize;

/// Models for the response data as defined by the stops API:
/// https://api-v3.mbta.com/docs/swagger/index.html#/Stop/ApiWeb_StopController_index
/// 
/// curl -X GET "https://api-v3.mbta.com/stops?filter[route]=Red" -H "accept: application/vnd.api+json" | jq  
/// {
///   "data": [
///     {
///       "attributes": {
///         "address": "Alewife Brook Pkwy and Cambridge Park Dr, Cambridge, MA 02140",
///         "at_street": null,
///         "description": null,
///         "latitude": 42.395428,
///         "location_type": 1,
///         "longitude": -71.142483,
///         "municipality": "Cambridge",
///         "name": "Alewife",
///         "on_street": null,
///         "platform_code": null,
///         "platform_name": null,
///         "vehicle_type": null,
///         "wheelchair_boarding": 1
///       },
///       "id": "place-alfcl",
///       "links": {
///         "self": "/stops/place-alfcl"
///       },
///       "relationships": {
///         "facilities": {
///           "links": {
///             "related": "/facilities/?filter[stop]=place-alfcl"
///           }
///         },
///         "parent_station": {
///           "data": null
///         },
///         "zone": {
///           "data": null
///         }
///       },
///       "type": "stop"
///     },
///  etc.

#[derive(Deserialize, Debug)]
pub struct Stops {
    pub data: Vec<Data>,
    pub jsonapi: Version,
}

impl fmt::Display for Stops {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jsonapi: {}", self.jsonapi.version)
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub attributes: Attributes,
    pub id: String,
    pub links: Option<Link>,
    pub relationships: Option<Relationships>,
    #[serde(rename = "type")]
    pub data_type: Option<String>,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stop: {}", self.attributes.name.clone().unwrap())
    }
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub address: Option<String>,
    pub at_street: Option<String>,
    pub description: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub municipality: Option<String>,
    pub name: Option<String>,
    pub on_street: Option<String>,
    pub platform_code: Option<String>,
    pub platform_name: Option<String>,
    pub vehicle_type: Option<String>,
    pub wheelchair_boarding: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: Option<String>,
    pub related: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub facilities: Option<Facilities>,
    pub parent_station: Option<ParentStation>,
    pub zone: Option<Zone>,
}

#[derive(Deserialize, Debug)]
pub struct Facilities {
    pub links: Option<Link>,
}

#[derive(Deserialize, Debug)]
pub struct ParentStation {
    pub data: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Zone {
    pub data: Option<ZoneData>,
}

#[derive(Deserialize, Debug)]
pub struct ZoneData {
    pub id: String,
    #[serde(rename = "type")]
    pub zone_data_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Line {
    pub data: LineData,
}

#[derive(Deserialize, Debug)]
pub struct LineData {
    pub id: String,
    #[serde(rename = "type")]
    pub line_data_type: u32,
}
