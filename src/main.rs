mod arq;
mod config;
mod langs;
mod utils;
mod presentation;
mod routes;
mod commands;
mod environment;


use presentation::cli_header::cli_header;
use presentation::help::help;
use std::env;
use crate::routes::router::router;

fn main() {
    let version = "2.0.0".to_string();
    let lang = "es".to_string();
    cli_header(&version, &lang);
    /*___ ARGS ___ */
    let args: Vec<String> = env::args().collect();
    router(&args,&lang, help);

    println!("Number of arguments: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }

}
