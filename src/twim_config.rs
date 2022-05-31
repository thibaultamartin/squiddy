use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use crate::{bot::Bot, bridge::Bridge};

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
            website: if bot.home.is_some() { bot.home.clone().unwrap() } else if bot.repository.is_some() { bot.repository.clone().unwrap() } else { "".to_string() },
            default_section: "bots".to_string(), 
            usual_reporters:  bot.authors.iter()
                                .filter_map(|author| author.matrix_id.clone()) 
                                .collect()
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
            website: if bridge.home.is_some() { bridge.home.clone().unwrap() } else if bridge.repository.is_some() { bridge.repository.clone().unwrap() } else { "".to_string() },
            default_section: "bridges".to_string(),
            usual_reporters:  bridge.authors.iter()
                                .filter_map(|author| author.matrix_id.clone()) 
                                .collect()
        }
    } 
}
