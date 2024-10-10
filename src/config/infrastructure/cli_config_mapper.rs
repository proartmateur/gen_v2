use std::env;

use super::super::domain::config::Config;
use super::super::domain::config_dto::ConfigDTO;

pub fn cli_config_mapper(dto: &ConfigDTO) -> Config {
    let current_dir = env::current_dir().unwrap();
    return Config { 
        headers_doc: dto.headers_doc.clone(),
        templates_root: dto.templates_root.clone(),
        current_dir: String::from(current_dir.to_str().unwrap()),
        arq_file: dto.arq_file.clone()
    };
}