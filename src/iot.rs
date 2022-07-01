use convert_case::{Case, Casing};
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::projects::Author;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Iot {
    pub title: String,
    pub description: String,
    pub authors: Vec<Author>,
    pub maturity: String,
    pub language: String,
    pub license: String,
    pub repository: Option<String>,
    pub home: Option<String>,
    pub screenshot: Option<String>,
    pub icon: Option<String>,
    pub room: Option<String>,
    pub featured: bool,
    pub sort_order: Option<i32>,
    pub full_description: String,
}

impl Iot {
    pub fn to_markdown(&self) -> String {
        let layout = match self.icon {
            Some(_) => "projectimage",
            None => "project",
        };
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
             - iot
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
