mod arq;
mod config;
mod langs;
mod presentation;
mod routes;

use arq::arq::Arq;
use config::infrastructure::fs_config_repository::FsConfigRepository;
use presentation::cli_header::cli_header;
use presentation::help::help;
use std::env;
use crate::routes::router::router;

fn main() {
    let version = "2.0.0".to_string();
    let lang = "es".to_string();
    cli_header(&version, &lang);

    let cfg_repo = FsConfigRepository { _init: () };
    let config = cfg_repo.read();
    print!("\ntemplates_root: {}\n", config.templates_root);
    println!("Current directory: {:?}\n", config.current_dir);

    /*___ ARGS ___ */
    let args: Vec<String> = env::args().collect();
    router(&args, config, help);

    println!("Number of arguments: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }

    /*___ ARGS ___ */

    /* __ Load Arq __ */
    /* 
    let path = "myarqu.json";

    match read_arq_json(path) {
        Ok(arq_items) => {
            println!("Successfully parsed Arq JSON: {:#?}", arq_items);
            // Test the function with a valid short_option
            if let Some(found_item) = find_arq_item_by_option(_arq, &args[1]) {
                println!("Found item by short_option: {:#?}", found_item);
            } else {
                println!("Item not found.");
            }
        }
        Err(e) => {
            eprintln!("Failed to read Arq JSON: {}", e);
            panic!("");
        }
    }
    */
}
