
mod presentation;
mod config;
mod langs;
mod arg_parser;


use config::infrastructure::fs_config_repository::FsConfigRepository;
use presentation::cli_header::cli_header;
use std::env;
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
}