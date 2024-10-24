use serde::{Deserialize, Serialize};
use super::arq_template::ArqTemplate;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArqItem {
    pub name: String,
    pub path: String,
    pub short_option: String,
    pub option: String,
    pub description: String,
    pub templates: Vec<ArqTemplate>,
    pub has_props: Option<bool>,
    pub prop_type_place: Option<usize>,
    pub prop_prop_place: Option<usize>,
    pub prop_prefix: Option<String>,
    pub prop_type_separator: Option<String>,
}
