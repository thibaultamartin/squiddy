use convert_case::{Case, Casing};
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::projects::Author;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    pub layout: String,             // e.g. "projectimage"
    pub title: String,              // e.g. "Element Web/Desktop"
    pub description: String, // e.g. "Element is a glossy web client with an emphasis on performance and usability"
    pub authors: Vec<Author>, // e.g. "Element"
    pub maturity: String,    // e.g. "Stable"
    pub language: String,    // e.g. "JavaScript"
    pub license: String,     // e.g. "Apache-2.0"
    pub repository: Option<String>, // e.g. "https://github.com/vector-im/element-web/"
    pub home: Option<String>, // e.g. "https://element.io/"
    pub screenshot: Option<String>, // e.g. "/docs/projects/images/riot-web-large.png"
    pub icon: Option<String>, // e.g. "/docs/projects/images/riot-web-small.png"
    pub room: Option<String>, // e.g. "#element-web:matrix.org"
    pub featured: bool,      // e.g. true
    pub sort_order: Option<i32>, // = 1
    pub full_description: String,
}

impl Server {
    pub fn to_markdown(&self) -> String {
        let layout = &self.layout;
        let title = &self.title;
        let description = &self.description;
        let authors = self
            .authors
            .iter()
            .map(|a| format!("{} {}", a.name, a.matrix_id.clone().unwrap_or_default()))
            .format(", ");
        let maturity = &self.maturity;
        let language = &self.language;
        let license = &self.license;
        let featured = &self.featured;

        let optional_fields = [
            self.repository.as_ref().map(|r| format!("repo: {r}")),
            self.home.as_ref().map(|h| format!("home: {h}")),
            self.screenshot.as_ref().map(|s| format!("screenshot: {s}")),
            self.icon.as_ref().map(|i| format!("thumbnail: {i}")),
            self.room.as_ref().map(|r| format!("room: \"{r}\"")),
            self.sort_order.as_ref().map(|o| format!("sort_order: {o}")),
        ]
        .iter()
        .flatten()
        .join("\n");

        let full_description = &self.full_description;

        formatdoc! {"
            ---
            layout: {layout}
            title: {title}
            categories:
             - server
            description: {description}
            author: {authors}
            maturity: {maturity}
            language: {language}
            license: {license}
            featured: {featured}
            {optional_fields}
            ---
            {full_description}
        "}
    }

    pub fn filename(&self) -> String {
        format!("{}.mdx", self.title.to_case(Case::Kebab))
    }
}
