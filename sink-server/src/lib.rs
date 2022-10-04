use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerData {
    pub forge_version: Option<String>,
    pub mods: Option<Vec<String>>,
}