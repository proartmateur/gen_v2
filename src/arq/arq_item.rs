use serde::{Deserialize, Serialize};

use super::arq_template::ArqTemplate;
// Define the ArqItem struct equivalent to the TypeScript interface
#[derive(Serialize, Deserialize, Debug)]
pub struct ArqItem {
    pub name: String,
    pub path: String,
    pub short_option: String,
    pub option: String,
    pub description: String,
    pub templates: Vec<ArqTemplate>,
    pub has_props: Option<bool>,
}
