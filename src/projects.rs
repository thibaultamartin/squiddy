use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::bot::Bot;
use crate::bridge::Bridge;
use crate::client::Client;
use crate::iot::Iot;
use crate::other::Other;
use crate::sdk::Sdk;
use crate::server::Server;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    pub matrix_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepositoryMetdata {
    pub url: String,
    // This is so that we could add other formats in the future
    pub appstream_location: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repositories {
    pub repositories: Vec<RepositoryMetdata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Projects {
    pub bots: Vec<Bot>,
    pub bridges: Vec<Bridge>,
    pub clients: Vec<Client>,
    pub iots: Vec<Iot>,
    pub others: Vec<Other>,
    pub sdks: Vec<Sdk>,
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Maturity {
    Obsolete,
    Alpha,
    Beta,
    Stable,
}

impl Display for Maturity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Maturity::Alpha => write!(f, "Alpha"),
            Maturity::Beta => write!(f, "Beta"),
            Maturity::Stable => write!(f, "Stable"),
            Maturity::Obsolete => write!(f, "Obsolete"),
        }
    }
}
