use std::fmt::{self, Display, Formatter};

use crate::bot::Bot;
use crate::bridge::Bridge;
use crate::client::Client;
use crate::iot::Iot;
use crate::other::Other;
use crate::sdk::Sdk;
use crate::server::Server;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    pub matrix_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Projects {
    pub bots: Vec<Bot>,
    pub bridges: Vec<Bridge>,
    pub clients: Vec<Client>,
    pub iots: Vec<Iot>,
    pub others: Vec<Other>,
    pub sdks: Vec<Sdk>,
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
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
