use std::process;
use crate::arq::arq_io::read_arq_json;
use crate::arq::arq_usecases::find_arq_item_by_option;
use crate::config::domain::config::Config;

pub fn router(args: &Vec<String>,cfg:Config, help_callback: fn() -> ()) {

    if args.len() < 2 {
        help_callback();
        process::exit(0x0100);
    }

    let first = args[1].clone();
    if (first.contains("--") || first.contains("-")) {
        // template
        let path = &cfg.arq_file;
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
                print!("{}","\nINIT ARQ\n");
            },
            _ =>{
                print!("{}","\nInvalid command\n");
            }
        }
    }
}
