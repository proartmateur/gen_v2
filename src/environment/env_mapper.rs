use super::env_vars::EnvVars;
use super::prop_style_mapper::prop_style_mapper;
use super::prop_vars::PropVars;
use crate::arq::arq_item::ArqItem;
use crate::config::domain::config::Config;
use crate::utils::string_transform::{snake_to_pascal_case, snake_to_camel_case};
use chrono::prelude::*;
use crate::environment::env_prop_mapper::env_prop_mapper;

pub fn env_mapper(arq_item: &ArqItem, name: &String, cfg: &Config, props: Option<String>) -> EnvVars {
    let mut attrs: Option<String> = None;
    let mut pretty_attrs: Option<String> = None;
    let mut props_env: Vec<PropVars> = vec![];
    let space = "*";

    match props {
        Some(p) => {
            
            let prop_style = prop_style_mapper(arq_item, space);
            
            props_env = env_prop_mapper(&p, &prop_style);
            
            attrs = Some(p.clone());
            let attrs_s = attrs.unwrap();
            /*if attrs_s.contains("_") {
                attrs = Some(attrs_s.replace("_", " "));
                attrs_s = attrs_s.replace("_", " ");
            } else {
                attrs = Some(attrs_s.clone());
            }*/
            attrs = Some(attrs_s.clone().replace(space, " "));
            pretty_attrs = Some(attrs_s.clone().replace(",", ",\n").replace(space, " "));
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
        props: props_env,
        author_name: cfg.author.clone(),
        author_email: cfg.author_email.clone(),
        now: Some(Utc::now().to_string()),
        path: arq_item.path.clone(),
        dq: String::from("\"")
    }
}