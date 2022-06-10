use convert_case::{Case, Casing};
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::projects::Author;
use crate::projects::Maturity;

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
pub enum Platform {
    Linux,
    Android,
    MacOS,
    IOS,
    Windows,
    DesktopWeb,
    MobileWeb,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub layout: String,             // e.g. "projectimage"
    pub id: String,                 // e.g. "element.io"
    pub title: String,              // e.g. "Element Web/Desktop"
    pub slug: Option<String>,       // e.g. "element"
    pub description: String, // e.g. "Element is a glossy web client with an emphasis on performance and usability"
    pub authors: Vec<Author>, // e.g. "Element"
    pub maturity: Maturity,    // e.g. "Stable"
    pub language: String,    // e.g. "JavaScript"
    pub license: String,     // e.g. "Apache-2.0"
    pub repository: Option<String>, // e.g. "https://github.com/vector-im/element-web/"
    pub home: Option<String>, // e.g. "https://element.io/"
    pub screenshot: Option<String>, // e.g. "/docs/projects/images/riot-web-large.png"
    pub icon: Option<String>, // e.g. "/docs/projects/images/riot-web-small.png"
    pub room: Option<String>, // e.g. "#element-web:matrix.org"
    pub sdk: Vec<String>,    // e.g. ["matrix-js-sdk", "matrix-react-sdk"]
    pub platforms: Vec<Platform>, // e.g. ["Linux", "macOS", "Windows", "DesktopWeb"]
    pub featured: bool,      // e.g. true
    pub sort_order: Option<i32>, // = 1
    pub features: Features,
    pub appstore_details: Option<AppStoreDetails>,
    pub apple_associated_app_id: Option<String>,
    pub playstore_app_id: Option<String>, // e.g. "im.vector.app"
    pub fdroid_app_id: Option<String>,    // e.g. "im.vector.app"
    pub flathub_app_id: Option<String>,   // e.g. ""
    pub otherinstall_link: Option<String>, // e.g. = ["https://element.io/get-started"]
    pub full_description: String,
}

impl Client {
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
        let sdk = self.sdk.iter().map(|s| format!("    - {}", s)).format("\n");
        let platforms = self
            .platforms
            .iter()
            .map(|p| {
                match p {
                    Platform::Linux => "    - Linux",
                    Platform::Android => "    - Android",
                    Platform::MacOS=> "    - macOS",
                    Platform::IOS => "    - iOS",
                    Platform::Windows => "    - Windows",
                    Platform::DesktopWeb => "    - Web",
                    Platform::MobileWeb => "    - Web",
            }})
            .format("\n");
        let featured = &self.featured;
        // Destructuring features to make the compiler scream if new fields are added and not rendered in markdown
        let Features {
            e2ee,
            widgets,
            spaces,
            room_directory,
            read_receipts,
            typing_indicators,
            edits,
            replies,
            threads,
            attachments,
            multi_account,
            registration,
            calls,
            reactions,
            sso,
            localised,
        } = &self.features;
        let full_description = &self.full_description;

        let optional_fields = [
            self.slug.as_ref().map(|s| format!("slug: {}", s)),
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

        formatdoc! {"
            ---
            layout: {layout}
            title: {title}
            categories:
             - client
            description: {description}
            author: {authors}
            maturity: {maturity}
            language: {language}
            license: {license}
            sdk:
            {sdk}
            platform:
            {platforms}
            featured: {featured}
            features:
                E2EE: {e2ee}
                Widgets: {widgets}
                Spaces: {spaces}
                Room directory: {room_directory}
                Read receipts: {read_receipts}
                Typing indicators: {typing_indicators}
                Edits: {edits}
                Replies: {replies}
                Threads: {threads}
                Attachments: {attachments}
                Multi accounts: {multi_account}
                Registration: {registration}
                Calls: {calls}
                Reactions: {reactions}
                SSO: {sso}
                Localised: {localised}
            {optional_fields}
            ---
            {full_description}
        "}
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

    pub fn matrixto_filename(&self) -> String {
        self
        .title
        .to_case(Case::Camel)
        .chars()
        .map(|x| match x {
            '/' => '-',
            '\\' => '-',
            _ => x,
        })
        .collect()
    }

    pub fn matrixto_data_file(&self) -> String {
        let id = &self.id;
        let name = &self.title;
        let description = &self.description;
        let maturity = format!("Maturity.{}", &self.maturity);

        let authors = self
            .authors
            .iter()
            .map(|a| format!("{} {}", a.name, a.matrix_id.clone().unwrap_or_default()))
            .format(", ");

        let platforms = self
            .platforms
            .iter()
            .map(|p| {
                match p {
                    Platform::Linux => "Platform.Linux",
                    Platform::Android => "Platform.Android",
                    Platform::MacOS=> "Platform.macOS",
                    Platform::IOS => "Platform.iOS",
                    Platform::Windows => "Platform.Windows",
                    Platform::DesktopWeb => "Platform.DesktopWeb",
                    Platform::MobileWeb => "Platform.MobileWeb",
                }
            })
            .join(", ");

        let optional_fields = [
            self.icon.as_ref().map(|i| format!("\"icon\": \"{}\"", i)),
            self.home.as_ref().map(|h| format!("\"home\": \"{}\"", h)),
            self.appstore_details.as_ref().map(|ad| format!("\"applestorelink\": new AppleStoreLink('{}', '{}')", ad.org, ad.app_id)),
            self.apple_associated_app_id.as_ref().map(|a| format!("\"appleAssociatedAppId\": \"{}\"", a)),
            self.playstore_app_id.as_ref().map(|p| format!("\"playstorelink\": new PlayStoreLink('{}')", p)),
            self.fdroid_app_id.as_ref().map(|f| format!("\"fdroidlink\": new FDroidLink('{}')", f)),
            self.flathub_app_id.as_ref().map(|f| format!("\"flathublink\": new FlathubLink('{}')", f)),
            self.otherinstall_link.as_ref().map(|o| format!("\"defaultInstallLink\": new WebsiteLink('{}')", o)),
        ]
        .iter()
        .flatten()
        .join(",\n");

        formatdoc!("
        import {{Maturity, Platform, FDroidLink, AppleStoreLink, PlayStoreLink, WebsiteLink, FlathubLink}} from \"../types.js\";

        export const data = {{
            \"id\": \"{id}\",
            \"platforms\": [{platforms}],
            \"name\": \"{name}\",
            \"description\": \"{description}\",
            \"author\": \"{authors}\",
            \"maturity\": \"{maturity}\",
            {optional_fields}
        }};")
    }
}
