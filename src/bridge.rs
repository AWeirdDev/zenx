use serde::Serialize;

#[derive(Serialize)]
pub struct Bridge {
    pub action: Action,
}

#[derive(Serialize)]
pub enum Action {
    #[serde(rename = "outline")]
    Outline,
}
