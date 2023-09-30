#![allow(dead_code, unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum IResult {
    Ok(String),
    Err(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
}
