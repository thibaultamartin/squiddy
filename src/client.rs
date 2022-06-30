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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Platform {
    Linux,
    Android,
    MacOS,
    Ios,
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
    pub maturity: Maturity,  // e.g. "Stable"
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
            .map(|p| match p {
                Platform::Linux => "    - Linux",
                Platform::Android => "    - Android",
                Platform::MacOS => "    - macOS",
                Platform::Ios => "    - iOS",
                Platform::Windows => "    - Windows",
                Platform::DesktopWeb => "    - Web",
                Platform::MobileWeb => "    - Web",
            })
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
        self.id
            .chars()
            .map(|c| match c {
                '/' => '-',
                '\\' => '-',
                '.' => '-',
                _ => c,
            })
            .collect::<String>()
            .to_case(Case::UpperCamel)
    }

    pub fn matrixto_join_file(id: String, clients: Vec<Client>) -> String {
        let filtered_clients: Vec<Client> = clients.into_iter().filter(|c| c.id == id).collect();

        let name = &id;
        let description = filtered_clients
            .clone()
            .into_iter()
            .find(|c| {
                c.platforms.contains(&Platform::DesktopWeb)
                    || c.platforms.contains(&Platform::Windows)
                    || c.platforms.contains(&Platform::MacOS)
            })
            .unwrap()
            .description;
        let maturity = format!(
            "Maturity.{}",
            filtered_clients // Get the lowest maturity of all clients
                .iter()
                .map(|c| c.maturity)
                .min()
                .unwrap()
        );

        let authors: String = if filtered_clients.len() > 1 {
            // If there's more than one client with the same id, return "{client_id} team"
            format!("{} team", &id)
        } else {
            filtered_clients[0]
                .authors
                .iter()
                .map(|a| format!("{} {}", a.name, a.matrix_id.clone().unwrap_or_default()))
                .join(", ")
        };

        let platforms = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.platforms)
            .unique()
            .collect::<Vec<Platform>>();

        let platforms = platforms
            .iter()
            .map(|p| match p {
                Platform::Linux => "Platform.Linux",
                Platform::Android => "Platform.Android",
                Platform::MacOS => "Platform.macOS",
                Platform::Ios => "Platform.iOS",
                Platform::Windows => "Platform.Windows",
                Platform::DesktopWeb => "Platform.DesktopWeb",
                Platform::MobileWeb => "Platform.MobileWeb",
            })
            .join(", ");

        // We're looking for the first Client that has the field we're interested in
        let icon = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.icon)
            .find(|_| true);

        let home = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.home)
            .find(|_| true);

        let appstore_details = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.appstore_details)
            .find(|_| true);

        let apple_associated_app_id = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.apple_associated_app_id)
            .find(|_| true);

        let playstore_app_id = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.playstore_app_id)
            .find(|_| true);

        let fdroid_app_id = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.fdroid_app_id)
            .find(|_| true);

        let flathub_app_id = filtered_clients
            .clone()
            .into_iter()
            .flat_map(|c| c.flathub_app_id)
            .find(|_| true);

        let otherinstall_link = filtered_clients
            .into_iter()
            .flat_map(|c| c.otherinstall_link)
            .find(|_| true);

        let optional_fields = [
            icon.as_ref().map(|i| format!("\"icon\": \"{}\"", i)),
            home.as_ref().map(|h| format!("\"home\": \"{}\"", h)),
            appstore_details.as_ref().map(|ad| {
                format!(
                    "\"applestorelink\": new AppleStoreLink('{}', '{}')",
                    ad.org, ad.app_id
                )
            }),
            apple_associated_app_id
                .as_ref()
                .map(|a| format!("\"appleAssociatedAppId\": \"{}\"", a)),
            playstore_app_id
                .as_ref()
                .map(|p| format!("\"playstorelink\": new PlayStoreLink('{}')", p)),
            fdroid_app_id
                .as_ref()
                .map(|f| format!("\"fdroidlink\": new FDroidLink('{}')", f)),
            flathub_app_id
                .as_ref()
                .map(|f| format!("\"flathublink\": new FlathubLink('{}')", f)),
            otherinstall_link
                .as_ref()
                .map(|o| format!("\"defaultInstallLink\": new WebsiteLink('{}')", o)),
        ]
        .iter()
        .flatten()
        .join(",\n    ");

        formatdoc!("
        import {{Maturity, Platform, FDroidLink, AppleStoreLink, PlayStoreLink, WebsiteLink, FlathubLink}} from \"../types.js\";
        export const data = {{
            \"id\": \"{name}\",
            \"platforms\": [{platforms}],
            \"name\": \"{name}\",
            \"description\": \"{description}\",
            \"author\": \"{authors}\",
            \"maturity\": {maturity},
            {optional_fields}
        }};")
    }

    pub fn matrixto_template_file(&self) -> String {
        format!(
            "/*
Copyright 2020 The Matrix.org Foundation C.I.C.
Licensed under the Apache License, Version 2.0 (the \"License\");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an \"AS IS\" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

import {{Platform, LinkKind}} from \"../types.js\";
import {{Client}} from \"./Client.js\";
import {{data}} from \"./{}-data.js\";

export class {} extends Client {{
    constructor() {{
        super(data);
    }}
}}",
            self.matrixto_filename(),
            self.matrixto_filename()
        )
    }

    pub fn matrixto_index(clients: Vec<Client>) -> String {
        let imports = clients
            .iter()
            .map(|c| {
                format!(
                    "import {{{}}} from \"./{}.js;\"",
                    c.matrixto_filename(),
                    c.matrixto_filename()
                )
            })
            .dedup()
            .join("\n");
        let instances = clients
            .iter()
            .map(|c| format!("new {}()", c.matrixto_filename()))
            .dedup()
            .join(",\n        ");

        format!(
            "/*
Copyright 2020 The Matrix.org Foundation C.I.C.
Licensed under the Apache License, Version 2.0 (the \"License\");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an \"AS IS\" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

{}

export function createClients() {{
    return [
        {}
    ];
}}",
            imports, instances
        )
    }
}
