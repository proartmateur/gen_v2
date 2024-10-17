use std::process;
use gen::arq::arq_io::read_arq_json;
use gen::arq::arq_usecases::find_arq_item_by_option;

//use super::super::config::domain::config::Config;
use gen::config::domain::config::Config;
use gen::environment::env_mapper::env_mapper;
pub fn option_process(args: &Vec<String>, config: &Config, help_callback: fn() -> ()) -> () {
    let first = args[1].clone();
    let path = &config.arq_file;
    let mut props = None;
    if args.len() >= 4 {
        props = Some(args[3].clone());
    }

    match read_arq_json(path) {
        Ok(arq_items) => {
            // Test the function with a valid short_option
            if let Some(found_item) = find_arq_item_by_option(&arq_items, &first) {
                println!("Found item by short_option: {:#?}", &found_item);
                let name = &args[2];
                let env_vars = env_mapper(&found_item, &name, config, props);
                print!("{:?}", env_vars);
            } else {
                println!("Option not found.");
                help_callback();
                process::exit(0x0100);
            }
        }
        Err(e) => {
            eprintln!("Failed to read Arq JSON: {}", e);
            panic!("");
        }
    }
}
