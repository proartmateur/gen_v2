use std::{fs, path::Path};

pub fn arq(file: &String) -> () {
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
    fs::write(file, example.to_string()).expect("some problems with save data");
    print!("{} is ready!\n", file);
    let template_path = Path::new("./").join(".gen_cli").join("templates");
    if !template_path.exists() {
      panic!(".gen_cli/templates not found.\n Try run: gen init")
    }
    let template_file = template_path.join("atom").join("component.tsx");
    let t_file = &template_file.display().to_string();
    print!("Writing: {}\n", &t_file);
    fs::create_dir_all(&template_file.parent().unwrap()).expect(format!("{}", "problem to make atom dir").as_str());
    fs::write(t_file, &empty_component.to_string()).expect("some problems with save data");
    print!("{} is ready!\n", &template_file.display());
}
