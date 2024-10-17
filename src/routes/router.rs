use std::process;
use crate::commands::arq::arq;
use crate::commands::init::init;
use crate::commands::option_process::option_process;
use crate::config::domain::config::Config;
use crate::config::infrastructure::fs_config_repository::FsConfigRepository;
use crate::langs::get_lang;
use crate::commands::man_vars::man_vars;

pub fn router(args: &Vec<String>, lang: &String, help_callback: fn() -> ()) {
    let strings = get_lang(lang);
    
    if args.len() < 2 {
        help_callback();
        process::exit(0x0100);
    }

    if args[1].contains("--") || args[1].contains("-") {
        // template
        let cfg_repo = FsConfigRepository { _init: () };
        let config: Config = cfg_repo.read();

        option_process(args, &config, help_callback);
        /*let path = &config.arq_file;
        let mut props = None;
        if args.len() >= 4 {
            props = Some(args[3].clone());
        }

        match read_arq_json(path) {
            Ok(arq_items) => {
                // Test the function with a valid short_option
                if let Some(found_item) = find_arq_item_by_option(&arq_items, &first) {
                    println!("Found item by short_option: {:#?}", &found_item);
                    let name =  &args[2];
                    let env_vars = env_mapper(&found_item, &name, &config, props);
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
        } */
    } else {
        // command
        match args[1].as_str() {
            "init" => {
                init(strings);
            },
            "arq" => {
                let cfg_repo = FsConfigRepository { _init: () };
                let config = cfg_repo.read();
                arq(&config.arq_file, strings);
            },
            "man:vars" => {
                man_vars();
            },
            _ =>{
                print!("{}","\nInvalid command\n");
            }
        }
        process::exit(0x0100);
    }
}
