use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PropStyle {
    pub type_separator: Option<char>,
    pub prop_place: usize,
    pub type_place: usize,
    pub prefix: Option<String>,
}