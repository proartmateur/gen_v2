
#[derive(Debug)]
pub struct PropVars {
    pub name: String,
    pub prop_type: String,
    pub prefix: String,
    pub type_separator: Option<String>,
    pub entity_name: String,
    pub snake_name: String,
    pub camel_name: String,
    pub kebab_name: String,
    pub constant_name: String,
}
