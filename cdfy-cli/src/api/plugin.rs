use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub id: Uuid,
    pub title: String,
    pub version: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlugin {
    pub title: String,
    pub version: String,
    pub url: String,
}
