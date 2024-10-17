//use std::collections::HashMap;

use super::prop_vars::PropVars;


#[derive(Debug)]
pub struct EnvVars {
    pub raw_name: String,
    pub entity_name: String,
    pub camel_name: String,
    pub snake_name: String,
    pub kebab_name: String,
    pub const_name: String,
    pub inline_props: Option<String>,
    pub pretty_props: Option<String>,
    pub props: Vec<PropVars>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub now: Option<String>,
    pub path: String,
    pub dq: String,
}