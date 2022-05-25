use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sdk {
    pub layout: String, // e.g. "projectimage"
    pub title: String, // e.g. "Element Web/Desktop"
    pub description: String, // e.g. "Element is a glossy web client with an emphasis on performance and usability"
    pub author: String, // e.g. "Element"
    pub maturity: String, // e.g. "Stable"
    pub language: String, // e.g. "JavaScript"
    pub licence: String, // e.g. "Apache-2.0"
    pub repository: String, // e.g. "https://github.com/vector-im/element-web/"
    pub home: String, // e.g. "https://element.io/"
    pub screenshot: String, // e.g. "/docs/projects/images/riot-web-large.png"
    pub icon: String, // e.g. "/docs/projects/images/riot-web-small.png"
    pub room: String, // e.g. "#element-web:matrix.org"
    pub featured: bool, // e.g. true
    pub sort_order: i32, // = 1
    pub full_description: String,
}