use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use crate::projects::Author;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bot {
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

impl Bot {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("---\n");
        markdown.push_str(&format!("layout: {}\n", self.layout));
        markdown.push_str(&format!("title: {}\n", self.title));
        markdown.push_str("categories:\n - bot\n");
        markdown.push_str(&format!("description: {}\n", self.description));
        markdown.push_str("author:");
        for author in &self.authors {
            markdown.push_str(&format!(
                " {} {},",
                author.name,
                author.matrix_id.clone().unwrap_or_else(|| "".to_string())
            ));
        }
        markdown.pop();
        markdown.push('\n');
        markdown.push_str(&format!("maturity: {}\n", self.maturity));
        markdown.push_str(&format!("language: {}\n", self.language));
        markdown.push_str(&format!("license: {}\n", self.license));
        if let Some::<String>(repository) = &self.repository {
            markdown.push_str(&format!("repo: {}\n", repository));
        }
        if let Some::<String>(home) = &self.home {
            markdown.push_str(&format!("home: {}\n", home));
        }
        if let Some::<String>(screenshot) = &self.screenshot {
            markdown.push_str(&format!("screenshot: {}\n", screenshot));
        }
        if let Some::<String>(icon) = &self.icon {
            markdown.push_str(&format!("thumbnail: {}\n", icon));
        }
        if let Some::<String>(room) = &self.room {
            markdown.push_str(&format!("room: \"{}\"\n", room));
        }
        markdown.push_str(&format!("featured: {}\n", self.featured));
        if let Some::<i32>(sort_order) = self.sort_order {
            markdown.push_str(&format!("sort_order: {}\n", sort_order));
        }
        markdown.push_str(&format!("---\n{}\n", self.full_description));

        markdown
    }

    pub fn filename(&self) -> String {
        format!("{}.mdx", self.title.to_case(Case::Kebab))
    }
}
