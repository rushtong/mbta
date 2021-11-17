use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Routes {
    pub data: Vec<Data>,
    pub jsonapi: Version,
}

impl fmt::Display for Routes {
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
        write!(f, "Route: {}", self.attributes.long_name)
    }
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub color: String,
    pub description: String,
    pub direction_destinations: Vec<String>,
    pub direction_names: Vec<String>,
    pub fare_class: String,
    pub long_name: String,
    pub short_name: String,
    pub sort_order: u32,
    pub text_color: String,
    #[serde(rename = "type")]
    pub attribute_type: u32,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub line: Option<Line>,
}

#[derive(Deserialize, Debug)]
pub struct Line {
    pub data: Option<LineData>,
}

#[derive(Deserialize, Debug)]
pub struct LineData {
    pub id: String,
    #[serde(rename = "type")]
    pub line_data_type: String,
}
