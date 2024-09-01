use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RenderComponent {
    pub id_selector: String,
    pub text_content: String,
    pub tag: String,
}
