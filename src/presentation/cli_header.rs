extern crate colored;
use colored::*;

use crate::langs::get_lang;

pub fn cli_header(version: &String, lang: &String) -> () {

    let strings = get_lang(lang);

    let app_name = "  Gen CLI  ".to_string();
    let app_version = format!("  {}  ", version);
    let author = "Enrique Nieto Martínez".to_string();
    let release_year = "2024".to_string();
    let description = strings["app_info.description"];
    let credits = strings["app_info.credits"]
        .replace("author", &author)
        .replace("year", &release_year);
    print!("\n\n");
    print!("{}", app_name.bold().on_truecolor(125, 70, 214));
    print!("{}", app_version.white().on_truecolor(95, 75, 139));
    print!("\n\n");
    print!("{}\n\n", description);
    print!("{}\n\n", credits.truecolor(164, 167, 193));
}
