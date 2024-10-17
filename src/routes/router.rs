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
