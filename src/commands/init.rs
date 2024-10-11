use std::{fs, path::Path, env};
use std::collections::HashMap;
use std::sync::LazyLock;

pub fn init(lang: &LazyLock<HashMap<&str, &str>> ) -> () {

    let template_dir = ".gen_cli/templates";
    if Path::new(&template_dir).exists() {
        print!("{} {}\n\n", &template_dir, lang["init.already_exists"]);
        return;
    }
    fs::create_dir_all(&template_dir).expect(format!("{} {}", lang["init.mkdir_error"], &template_dir).as_str());

    let cfg_file ="gen_config.json";
    if Path::new(&cfg_file).exists() {
        print!("{} {}\n\n", &cfg_file, lang["init.already_exists"]);
        return;
    }
    let cwd = env::current_dir().unwrap();
    let template_root = Path::new(&cwd.display().to_string()).join(&template_dir);
    let cfg: &'static str = "{
    \"headers_doc\": false,
    \"author\": \"myself\",
    \"author_email\": \"myself@example.com\",
    \"templates_root\": \"$CWD$\",
    \"arq_file\":\"arq.json\"
}";
    let contents = cfg.replace("$CWD$", &template_root.display().to_string());
    fs::write(&cfg_file, contents).expect(format!("{} {}",lang["init.write_file_error"], &cfg_file).as_str());
    print!("\n{}\n\n\n",lang["init.ready_for_init"]);
}