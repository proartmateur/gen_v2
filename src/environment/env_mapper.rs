use super::env_vars::EnvVars;
use crate::arq::arq_item::ArqItem;
//use crate::config::domain::config::Config;
//use super::super::config::domain::config::Config;
use crate::config::domain::config::Config;
use crate::utils::string_transform::{snake_to_pascal_case, snake_to_camel_case};
use chrono::prelude::*;

pub fn env_mapper(template: &ArqItem, name: &String, cfg: &Config, props: Option<String>) -> EnvVars {

    let mut attrs: Option<String> = None;
    let mut pretty_attrs: Option<String> = None;

    match props {
        Some(p) => {
            attrs = Some(p.clone());
            let mut attrs_s = attrs.unwrap();
            if attrs_s.contains("_") {
                attrs = Some(attrs_s.replace("_", " "));
                attrs_s = attrs_s.replace("_", " ");
            } else {
                attrs = Some(attrs_s.clone());
            }
            pretty_attrs = Some(attrs_s.clone().replace(",", ",\n"));
        },
        None => {}
    }

    //TODO Prop Separation
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
        author_name: cfg.author.clone(),
        author_email: cfg.author_email.clone(),
        now: Some(Utc::now().to_string()),
        path: template.path.clone(),
        dq: String::from("\"")
    }
}