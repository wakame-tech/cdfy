use fp_bindgen::prelude::Serializable;

#[derive(Serializable)]
pub enum IResult {
    Ok(String),
    Err(String),
}

#[derive(Serializable)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
}
