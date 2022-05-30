use serde::{Deserialize, Serialize};

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