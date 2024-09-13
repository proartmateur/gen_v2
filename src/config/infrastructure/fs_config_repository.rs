use std::path::Path;
extern crate colored;
use colored::*;

use super::cli_config_mapper::cli_config_mapper;
use crate::config::domain::{config::Config, config_dto::ConfigDTO, read_config_error::ConfigError};

pub struct FsConfigRepository {
    pub(crate) _init: (),
}

impl FsConfigRepository {
    pub fn read(&self) -> Config {
        let data = std::fs::read_to_string("gen_config.json");
        match data {
            Ok(value) => {
                //let datas = data.unwrap();
                let config: ConfigDTO = serde_json::from_str(&value).unwrap();
                validate_existence(&config.templates_root);
                return cli_config_mapper(&config);
            }
            Err(error) => {
                print!("{}", ConfigError::ReadError("{gen_config.json} not exists :(".to_string()));
                panic!("{}", error.to_string().red());
            }
        }
    }
}

fn validate_existence(needle_path: &String) -> () {
    let path = Path::new(needle_path);

    if !path.exists() {
        panic!("{}", ConfigError::ReadError(format!("{} not exists :(", needle_path)));
    } 
}