use std::process;
use crate::arq::arq_io::read_arq_json;
use crate::arq::arq_usecases::find_arq_item_by_option;
use crate::commands::arq::arq;
use crate::commands::init::init;
use crate::config::infrastructure::fs_config_repository::FsConfigRepository;
use crate::langs::get_lang;

pub fn router(args: &Vec<String>, lang: &String, help_callback: fn() -> ()) {
    let strings = get_lang(lang);
    
    if args.len() < 2 {
        help_callback();
        process::exit(0x0100);
    }

    let first = args[1].clone();
    if first.contains("--") || first.contains("-") {
        // template
        let cfg_repo = FsConfigRepository { _init: () };
        let config = cfg_repo.read();
        let path = &config.arq_file;
        match read_arq_json(path) {
            Ok(arq_items) => {
                //println!("Successfully parsed Arq JSON: {:#?}", arq_items);
                // Test the function with a valid short_option
                if let Some(found_item) = find_arq_item_by_option(&arq_items, &first) {
                    println!("Found item by short_option: {:#?}", found_item);
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
    } else {
        // command
        match first.as_str() {
            "init" => {
                init(strings);
            },
            "arq" => {
                let cfg_repo = FsConfigRepository { _init: () };
                let config = cfg_repo.read();
                arq(&config.arq_file, strings);
            },
            _ =>{
                print!("{}","\nInvalid command\n");
            }
        }
        process::exit(0x0100);
    }
}
