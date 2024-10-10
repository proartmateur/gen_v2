use std::{fs, path::Path};

pub fn init(file: &String) -> () {
    if Path::new(file).exists() {
        print!("{} {}\n\n", file, "already exists!");
        return;
    }

    let example: &'static str = "[
  {
    \"name\": \"atom\",
    \"path\": \"/src/atoms\",
    \"short_option\": \"-a\",
    \"option\": \"--atom\",
    \"description\": \"frontend atom component\",
    \"templates\": [
      {
        \"template\": \"/atom/component.jsx\",
        \"destination\": \"<path>/<Ent>.jsx\"
      }
    ]
  }
]";
    fs::write(file, example.to_string()).expect("some problems with save data");
    print!("{} is ready!",file);
}
