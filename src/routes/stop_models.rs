use std::fmt;

use serde::Deserialize;

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
