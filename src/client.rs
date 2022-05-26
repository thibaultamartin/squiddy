use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub layout: String, // e.g. "projectimage"
    pub id: String, // e.g. "element.io"
    pub title: String, // e.g. "Element Web/Desktop"
    pub slug: Option<String>, // e.g. "element"
    pub description: String, // e.g. "Element is a glossy web client with an emphasis on performance and usability"
    pub author: String, // e.g. "Element"
    pub maturity: String, // e.g. "Stable"
    pub language: String, // e.g. "JavaScript"
    pub licence: String, // e.g. "Apache-2.0"
    pub repository: Option<String>, // e.g. "https://github.com/vector-im/element-web/"
    pub home: Option<String>, // e.g. "https://element.io/"
    pub screenshot: Option<String>, // e.g. "/docs/projects/images/riot-web-large.png"
    pub icon: Option<String>, // e.g. "/docs/projects/images/riot-web-small.png"
    pub room: Option<String>, // e.g. "#element-web:matrix.org"
    pub sdk: Vec<String>, // e.g. ["matrix-js-sdk", "matrix-react-sdk"] 
    pub platforms: Vec<String>, // e.g. ["Linux", "macOS", "Windows", "DesktopWeb"]
    pub featured: bool, // e.g. true
    pub sort_order: Option<i32>, // = 1
    pub features: Vec<String>, // e.g. ["a", "b"]
    //pub appstore_details = { org = "vector", app_id = "id1083446067" },
    pub playstore_app_id: Option<String>, // e.g. "im.vector.app"
    pub fdroid_app_id: Option<String>, // e.g. "im.vector.app"
    pub flathub_app_id: Option<String>, // e.g. ""
    pub otherinstall_link: Option<Vec<String>>, // e.g. = ["https://element.io/get-started"]
    pub full_description: String,
}

impl Client {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("---\n");
        markdown.push_str(
            &format!("layout: {}\n", self.layout)
        );
        markdown.push_str(
            &format!("title: {}\n", self.title)
        );
        if let Some::<String>(slug) = &self.slug {
            markdown.push_str(
                &format!("slug: {}\n", slug)
            ); 
        }
        markdown.push_str("categories:\n - client\n");
        markdown.push_str(
            &format!("description: {}\n", self.description)
        );
        markdown.push_str(
            &format!("author: {}\n", self.author)
        );
        markdown.push_str(
            &format!("maturity: {}\n", self.maturity)
        );
        markdown.push_str(
            &format!("language: {}\n", self.language)
        );
        markdown.push_str(
            &format!("licence: {}\n", self.licence)
        );
        if let Some::<String>(repository) = &self.repository {                
            markdown.push_str(
                &format!("repository: {}\n", repository)
            );
        }
        if let Some::<String>(home) = &self.home {
            markdown.push_str(
                &format!("home: {}\n", home)
            );
        }
        if let Some::<String>(screenshot) = &self.screenshot {
            markdown.push_str(
                &format!("screenshot: {}\n", screenshot)
            );
        }
        if let Some::<String>(icon) = &self.icon {
            markdown.push_str(
                &format!("thumbnail: {}\n", icon)
            );
        }
        if let Some::<String>(room) = &self.room {
            markdown.push_str(
                &format!("room: {}\n", room)
            );
        }
        markdown.push_str("SDK: ");
        for sdk in &self.sdk {
            markdown.push_str(
                &format!("{}, ", sdk)
            );
        }
        markdown.pop();
        markdown.pop();
        markdown.push('\n');
        markdown.push_str("platforms:\n");
        for platform in &self.platforms {
            markdown.push_str(
                &format!("    - {}\n", platform)
            );
        }
        markdown.push_str(
            &format!("featured: {}\n", self.featured)
        );
        if let Some::<i32>(sort_order) = self.sort_order {
            markdown.push_str(
                &format!("sort_order: {}\n", sort_order)
            );
        }
        // features
        panic!("Features are not rendered");
        markdown.push_str(
            &format!("---\n{}\n", self.full_description)
        );

        markdown
    }
}