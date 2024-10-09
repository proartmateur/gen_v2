use serde::{Deserialize, Serialize};

// Define the ArqTemplate struct equivalent to the TypeScript interface
#[derive(Serialize, Deserialize, Debug)]
pub struct ArqTemplate {
    pub template: String,
    pub destination: String,
    pub per_prop: Option<bool>,
    pub prop_naming: Option<String>,
    pub per_prop_import: Option<String>,
}
