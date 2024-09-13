use crate::config::domain::{config::Config, config_dto::ConfigDTO};

use super::cli_config_mapper::cli_config_mapper;

pub struct FsConfigRepository {
    pub(crate) _init: ()
}

impl FsConfigRepository {
    pub fn read(&self) -> Config {
        let data = std::fs::read_to_string("gen_config.json");
        let datas = data.unwrap();
        let config: ConfigDTO = serde_json::from_str(&datas).unwrap();
        return cli_config_mapper(&config);
    }
}
