extern crate colored;
use colored::*;

//use crate::langs::get_lang;

pub fn help() {
    let head = "======= HELP =======";
    print!("{}\n\n", head.to_string().bold().on_truecolor(125, 70, 214));
    let top = "How to use?\n\n\
              gen <COMMAND>\n\n\
              or\n\n\
              gen <OPTION> <PROPS>";
    
    print!("{}\n\n", top);
    print!("{}\n\n", "  Commands  ".to_string().bold().on_truecolor(214, 204, 184));
    print!("{}  {}\n", "  init".to_string().bold().yellow(), "Create an arq.json file");
    print!("\n\n");
}