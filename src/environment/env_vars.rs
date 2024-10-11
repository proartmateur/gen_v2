use std::collections::HashMap;

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
    pub snake_separated_props: Option<HashMap<String, String>>,
    pub entity_separated_props: Option<HashMap<String, String>>,
    pub camel_separated_props: Option<HashMap<String, String>>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub now: Option<String>,
}