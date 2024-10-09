mod presentation;
mod config;
mod langs;
mod arq;
use config::infrastructure::fs_config_repository::FsConfigRepository;
use presentation::cli_header::cli_header;
use std::env;

use crate::arq::arq_io::read_arq_json;
use crate::arq::arq_usecases::find_arq_item_by_option;
fn main() {
    let version = "2.0.0".to_string();
    let lang = "es".to_string();
    cli_header(&version, &lang);

    let cfg_repo = FsConfigRepository{ _init:() };
    let config = cfg_repo.read();
    print!("\ntemplates_root: {}\n", config.templates_root);
    println!("Current directory: {:?}\n", config.current_dir);


    
    
    /*___ ARGS ___ */
    let args: Vec<String> = env::args().collect();

    println!("Number of arguments: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }


    /*___ ARGS ___ */

    /* __ Load Arq __ */
    let path = "myarq.json";
    match read_arq_json(path) {
        Ok(arq_items) => {
            println!("Successfully parsed Arq JSON: {:#?}", arq_items);
            // Test the function with a valid short_option
            if let Some(found_item) = find_arq_item_by_option(
                &arq_items, &args[1]) {
                println!("Found item by short_option: {:#?}", found_item);
            } else {
                println!("Item not found.");
            }
            
        }
        Err(e) => {
            eprintln!("Failed to read Arq JSON: {}", e);
        }
    }

}