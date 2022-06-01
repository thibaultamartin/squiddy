use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use crate::projects::Author;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Features {
    pub e2ee: String,
    pub widgets: String,
    pub spaces: String,
    pub room_directory: String,
    pub read_receipts: String,
    pub typing_indicators: String,
    pub edits: String,
    pub replies: String,
    pub threads: String,
    pub attachments: String,
    pub multi_account: String,
    pub registration: String,
    pub calls: String,
    pub reactions: String,
    pub sso: String,
    pub localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppStoreDetails {
    pub org: String,
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub layout: String,             // e.g. "projectimage"
    pub id: String,                 // e.g. "element.io"
    pub title: String,              // e.g. "Element Web/Desktop"
    pub slug: Option<String>,       // e.g. "element"
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
    pub sdk: Vec<String>,    // e.g. ["matrix-js-sdk", "matrix-react-sdk"]
    pub platforms: Vec<String>, // e.g. ["Linux", "macOS", "Windows", "DesktopWeb"]
    pub featured: bool,      // e.g. true
    pub sort_order: Option<i32>, // = 1
    pub features: Features,
    pub appstore_details: Option<AppStoreDetails>,
    pub playstore_app_id: Option<String>, // e.g. "im.vector.app"
    pub fdroid_app_id: Option<String>,    // e.g. "im.vector.app"
    pub flathub_app_id: Option<String>,   // e.g. ""
    pub otherinstall_link: Option<Vec<String>>, // e.g. = ["https://element.io/get-started"]
    pub full_description: String,
}

impl Client {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("---\n");
        markdown.push_str(&format!("layout: {}\n", self.layout));
        markdown.push_str(&format!("title: {}\n", self.title));
        if let Some::<String>(slug) = &self.slug {
            markdown.push_str(&format!("slug: {}\n", slug));
        }
        markdown.push_str("categories:\n - client\n");
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
        markdown.push_str("SDK: ");
        for sdk in &self.sdk {
            markdown.push_str(&format!("{}, ", sdk));
        }
        markdown.pop();
        markdown.pop();
        markdown.push('\n');
        markdown.push_str("platforms:\n");
        for platform in &self.platforms {
            markdown.push_str(&format!("    - {}\n", platform));
        }
        markdown.push_str(&format!("featured: {}\n", self.featured));
        if let Some::<i32>(sort_order) = self.sort_order {
            markdown.push_str(&format!("sort_order: {}\n", sort_order));
        }
        markdown.push_str("features:\n");
        markdown.push_str(&format!("    e2ee: {}\n", self.features.e2ee));
        markdown.push_str(&format!("    widgets: {}\n", self.features.widgets));
        markdown.push_str(&format!("    spaces: {}\n", self.features.spaces));
        markdown.push_str(&format!(
            "    room_directory: {}\n",
            self.features.room_directory
        ));
        markdown.push_str(&format!(
            "    read_receipts: {}\n",
            self.features.read_receipts
        ));
        markdown.push_str(&format!(
            "    typing_indicators: {}\n",
            self.features.typing_indicators
        ));
        markdown.push_str(&format!("    edits: {}\n", self.features.edits));
        markdown.push_str(&format!("    replies: {}\n", self.features.replies));
        markdown.push_str(&format!("    threads: {}\n", self.features.threads));
        markdown.push_str(&format!("    attachments: {}\n", self.features.attachments));
        markdown.push_str(&format!(
            "    multi_account: {}\n",
            self.features.multi_account
        ));
        markdown.push_str(&format!(
            "    registration: {}\n",
            self.features.registration
        ));
        markdown.push_str(&format!("    calls: {}\n", self.features.calls));
        markdown.push_str(&format!("    reactions: {}\n", self.features.reactions));
        markdown.push_str(&format!("    sso: {}\n", self.features.sso));
        markdown.push_str(&format!("    localised: {}\n", self.features.localised));
        markdown.push_str(&format!("---\n{}\n", self.full_description));

        markdown
    }

    pub fn filename(&self) -> String {
        let normalised_name: String = self
            .title
            .to_case(Case::Kebab)
            .chars()
            .map(|x| match x {
                '/' => '-',
                '\\' => '-',
                _ => x,
            })
            .collect();
        format!("{}.mdx", normalised_name)
    }
}
