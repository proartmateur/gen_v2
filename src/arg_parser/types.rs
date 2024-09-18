use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionData {
    pub short_option: String,
    pub option: String,
    pub template: String,
    #[serde(default)]
    pub has_props: bool,
}

#[derive(Debug, Serialize)]
pub struct OptionResult {
    pub short_option: String,
    pub option: String,
    pub template: String,
    pub params: Value,
}

#[derive(Debug, Serialize)]
pub struct ArgRouteError {
    pub message: String,
}