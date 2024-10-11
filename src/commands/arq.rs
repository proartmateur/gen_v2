use std::{fs, path::Path};
use std::collections::HashMap;
use std::sync::LazyLock;

pub fn arq(file: &String, lang: &LazyLock<HashMap<&str, &str>>) -> () {
    if Path::new(file).exists() {
        print!("{} {}\n\n", file, lang["init.already_exists"]);
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
        \"template\": \"/atom/component.tsx\",
        \"destination\": \"<path>/<Ent>.tsx\"
      }
    ]
  }
]";

    let empty_component: &'static str = "{$FILE_HEADER$}

export interface $ENT$Props{}
export function $ENT$({}:$ENT$Props){
    return <></>
}";
    fs::write(file, example.to_string()).expect(format!("{}",lang["init.write_file_error"]).as_str());
    print!("{} {}\n", file, lang["arq.is_ready"]);
    let template_path = Path::new("./").join(".gen_cli").join("templates");
    if !template_path.exists() {
      panic!("{}",lang["arq.panic_gen_dir"])
    }
    let template_file = template_path.join("atom").join("component.tsx");
    let t_file = &template_file.display().to_string();
    print!("{} {}\n",lang["arq.writing"], &t_file);
    fs::create_dir_all(&template_file.parent().unwrap()).expect(format!("{}", lang["arq.error_make_atom_dir"]).as_str());
    fs::write(t_file, &empty_component.to_string()).expect(format!("{}",lang["init.write_file_error"]).as_str());
    print!("{} {}\n", &template_file.display(),lang["arq.is_ready"]);
}
