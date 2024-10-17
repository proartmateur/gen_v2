use gen::presentation::cli_header::cli_header;
use gen::presentation::help::help;
use std::env;
use gen::routes::router::router;

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
