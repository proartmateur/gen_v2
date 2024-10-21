use std::process;
use crate::arq::arq_io::read_arq_json;
use crate::arq::arq_usecases::find_arq_item_by_option;
use crate::config::domain::config::Config;
use crate::environment::env_mapper::env_mapper;

pub fn option_process(args: &Vec<String>, config: &Config, help_callback: fn() -> ()) -> () {
    let first = args[1].clone();
    let path = &config.arq_file;
    let mut props = None;
    if args.len() >= 4 {
        props = Some(args[3].clone());
    }


    match read_arq_json(path) {
        Ok(arq_items) => {
            if let Some(found_item) = find_arq_item_by_option(&arq_items, &first) {
                
                // Get Env Vars with props - ✅
                // Write a template replacer that change vars for env_vars_values
                //   V1 Replace only Entity vars and inline_props and pretty props
                //   v2 Replace prop syntax
                // For Each Template get Template & Replace
                //     Write file

                println!("\nFound item by short_option: {:#?}\n\n", &found_item);
                let name = &args[2];
                let env_vars = env_mapper(&found_item, &name, config, props.clone());
                print!("\n{:#?}\n\n", env_vars);

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
