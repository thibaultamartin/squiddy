use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub layout: String, // e.g. "projectimage"
    pub id: String, // e.g. "element.io"
    pub title: String, // e.g. "Element Web/Desktop"
    pub slug: String, // e.g. "element"
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
    pub sdk: Vec<String>, // e.g. ["matrix-js-sdk", "matrix-react-sdk"] 
    pub platforms: Vec<String>, // e.g. ["Linux", "macOS", "Windows", "DesktopWeb"]
    pub featured: bool, // e.g. true
    pub sort_order: i32, // = 1
    pub features: Vec<String>, // e.g. ["a", "b"]
    //pub appstore_details = { org = "vector", app_id = "id1083446067" },
    pub playstore_app_id: String, // e.g. "im.vector.app"
    pub fdroid_app_id: String, // e.g. "im.vector.app"
    pub flathub_app_id: String, // e.g. ""
    pub otherinstall_link: Vec<String>, // e.g. = ["https://element.io/get-started"]
    pub full_description: String,
}