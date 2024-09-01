use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bridge {
    pub action: Action,
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "outline_request")]
    OutlineRequest,

    #[serde(rename = "outline")]
    Outline(Vec<Component>),
}

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub tag: String,
    pub text_content: String,
    pub maps: Option<HashMap<String, String>>,
}
