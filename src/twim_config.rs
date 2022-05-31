use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use crate::{bot::Bot, bridge::Bridge, client::Client, iot::Iot, other::Other, sdk::Sdk, server::Server};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Project {
    pub emoji: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub website: String,
    pub default_section: String,
    pub usual_reporters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Section {
    pub emoji: String,
    pub name: String,
    pub title: String,
    pub order: u32,
    pub usual_reporters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub bot_user_id: String,
    pub reporting_room_id: String,
    pub admin_room_id: String,
    pub notice_emoji: String,
    pub image_markdown: String,
    pub video_markdown: String,
    pub verbs: Vec<String>,
    pub update_config_command: String,
    pub editors: Vec<String>,
    pub sections: Vec<Section>,
    pub projects: Vec<Project>,
}

impl From<&Bot> for Project {
    fn from(bot: &Bot) -> Self {
        Project {
            emoji: format!("{}?", bot.title.to_case(Case::Kebab)),
            name: bot.title.to_case(Case::Kebab),
            title: bot.title.clone(),
            description: bot.description.clone(),
            website: bot
                .home
                .clone()
                .or_else(|| bot.repository.clone())
                .unwrap_or_default(),
            default_section: "bots".to_string(),
            usual_reporters: bot
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}

impl From<&Bridge> for Project {
    fn from(bridge: &Bridge) -> Self {
        Project {
            emoji: format!("{}?", bridge.title.to_case(Case::Kebab)),
            name: bridge.title.to_case(Case::Kebab),
            title: bridge.title.clone(),
            description: bridge.description.clone(),
            website: bridge
                .home
                .clone()
                .or_else(|| bridge.repository.clone())
                .unwrap_or_default(),
            default_section: "bridges".to_string(),
            usual_reporters: bridge
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}

impl From<&Client> for Project {
    fn from(client: &Client) -> Self {
        Project {
            emoji: format!("{}?", client.title.to_case(Case::Kebab)),
            name: client.title.to_case(Case::Kebab),
            title: client.title.clone(),
            description: client.description.clone(),
            website: client
                .home
                .clone()
                .or_else(|| client.repository.clone())
                .unwrap_or_default(),
            default_section: "clients".to_string(),
            usual_reporters: client
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}

impl From<&Iot> for Project {
    fn from (iot: &Iot) -> Self {
        Project {
            emoji: format!("{}?", iot.title.to_case(Case::Kebab)),
            name: iot.title.to_case(Case::Kebab),
            title: iot.title.clone(),
            description: iot.description.clone(),
            website: iot
                .home
                .clone()
                .or_else(|| iot.repository.clone())
                .unwrap_or_default(),
            default_section: "iot".to_string(),
            usual_reporters: iot
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}

impl From<&Other> for Project {
    fn from (other: &Other) -> Self {
        Project {
            emoji: format!("{}?", other.title.to_case(Case::Kebab)),
            name: other.title.to_case(Case::Kebab),
            title: other.title.clone(),
            description: other.description.clone(),
            website: other
                .home
                .clone()
                .or_else(|| other.repository.clone())
                .unwrap_or_default(),
            default_section: "other".to_string(),
            usual_reporters: other
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}

impl From<&Sdk> for Project {
    fn from (sdk: &Sdk) -> Self {
        Project {
            emoji: format!("{}?", sdk.title.to_case(Case::Kebab)),
            name: sdk.title.to_case(Case::Kebab),
            title: sdk.title.clone(),
            description: sdk.description.clone(),
            website: sdk
                .home
                .clone()
                .or_else(|| sdk.repository.clone())
                .unwrap_or_default(),
            default_section: "sdks".to_string(),
            usual_reporters: sdk
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}

impl From<&Server> for Project {
    fn from (server: &Server) -> Self {
        Project {
            emoji: format!("{}?", server.title.to_case(Case::Kebab)),
            name: server.title.to_case(Case::Kebab),
            title: server.title.clone(),
            description: server.description.clone(),
            website: server
                .home
                .clone()
                .or_else(|| server.repository.clone())
                .unwrap_or_default(),
            default_section: "servers".to_string(),
            usual_reporters: server
                .authors
                .iter()
                .filter_map(|author| author.matrix_id.clone())
                .collect(),
        }
    }
}