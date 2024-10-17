use std::env;
use crate::config::domain::{config::Config, config_dto::ConfigDTO};
pub fn cli_config_mapper(dto: &ConfigDTO) -> Config {
    let current_dir = env::current_dir().unwrap();
    return Config { 
        templates_root: dto.templates_root.clone(),
        author: dto.author.clone(),
        author_email: dto.author_email.clone(),
        current_dir: String::from(current_dir.to_str().unwrap()),
        arq_file: dto.arq_file.clone()
    };
}