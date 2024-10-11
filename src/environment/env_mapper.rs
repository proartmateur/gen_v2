use super::env_vars::EnvVars;
use crate::arq::arq_item::ArqItem;
use crate::config::domain::config::Config;
use crate::utils::string_transform::{snake_to_pascal_case, snake_to_camel_case};
use chrono::prelude::*;

pub fn env_mapper(template: &ArqItem, name: &String, cfg: &Config, props: Option<String>) -> EnvVars {

    let mut attrs: Option<String> = None;
    let mut pretty_attrs: Option<String> = None;

    match props {
        Some(p) => {
            attrs = Some(p.clone());
            pretty_attrs = Some(p.clone().replace(",", ",\n"));
        },
        None => {}
    }
    return EnvVars {
        raw_name: name.clone(),
        entity_name: snake_to_pascal_case(&name.to_lowercase().as_str()),
        camel_name: snake_to_camel_case(&name.to_lowercase().as_str()),
        snake_name: name.to_lowercase(),
        kebab_name: name.to_lowercase().replace("_", "-"),
        const_name: name.to_uppercase(),
        inline_props: attrs,
        pretty_props: pretty_attrs,
        snake_separated_props: None,
        entity_separated_props: None,
        camel_separated_props: None,
        author_name: Some("".to_string()),
        author_email: Some("".to_string()),
        now: Some(Utc::now().to_string()),
    }
}