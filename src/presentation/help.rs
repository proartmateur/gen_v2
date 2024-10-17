extern crate colored;
use colored::*;
//TODO: Implement i18n
pub fn help() {
    let head = "======= HELP =======";
    print!("{}\n\n", head.to_string().bold().on_truecolor(125, 70, 214));
    let top = "How to use?\n\n\
              gen <COMMAND>\n\n\
              or\n\n\
              gen <OPTION> <PROPS>";

    print!("{}\n\n", top);
    print!(
        "{}\n\n",
        "  Commands  "
            .to_string()
            .bold()
            .on_truecolor(214, 204, 184)
    );
    print!(
        "{}  {}\n",
        "  init".to_string().bold().yellow(),
        "Create a gen_config.json file and .gen_cli folder"
    );
    print!(
        "{}  {}\n",
        "  arq".to_string().bold().yellow(),
        "Create an arq.json file"
    );
    print!("\n\n");
}
