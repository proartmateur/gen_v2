mod config;
use config::infrastructure::fs_config_repository::FsConfigRepository;
use std::env;

fn main() {
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