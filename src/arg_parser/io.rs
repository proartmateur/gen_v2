use crate::arg_parser::types::OptionData;
use std::collections::HashMap;
use std::fs;


pub fn read_options_from_file(filename: &str) -> Result<Vec<OptionData>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let options: Vec<OptionData> = serde_json::from_str(&content)?;
    Ok(options)
}


pub fn index_options(options: &[OptionData]) -> (HashMap<String, &OptionData>, HashMap<String, &OptionData>) {

    let mut short_options = HashMap::new();
    let mut long_options = HashMap::new();

    for option in options {
        short_options.insert(option.short_option.clone(), option);
        long_options.insert(option.option.clone(), option);
    }

    (short_options, long_options)
}

