use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDTO {
    pub headers_doc: bool,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub templates_root: String,
    pub arq_file: String,
}