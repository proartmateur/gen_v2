use std::{fs, io::{self, Write}, path::Path, process};
use crate::arq::arq_io::read_arq_json;
use crate::arq::arq_usecases::find_arq_item_by_option;
use crate::config::domain::config::Config;
use crate::environment::env_mapper::env_mapper;
use crate::replacer::template_replacer::template_replacer;
use crate::langs::get_lang;

pub fn option_process(args: &Vec<String>, config: &Config, help_callback: fn() -> (), lang: &String) -> () {
    let first = args[1].clone();
    let path = &config.arq_file;
    let mut props = None;
    if args.len() >= 4 {
        props = Some(args[3].clone());
    }

    let lang_strings = get_lang(lang);

    match read_arq_json(path) {
        Ok(arq_items) => {
            if let Some(found_item) = find_arq_item_by_option(&arq_items, &first) {
                
                let name = &args[2];
                let env_vars = env_mapper(&found_item, &name, config, props.clone());

                // Process templates
                let mut imports_to_add: Vec<String> = Vec::new();
                
                // First pass: collect imports from per_prop templates
                for template_config in &found_item.templates {
                    if template_config.per_prop == Some(true) {
                        // Collect imports for each property
                        for prop in &env_vars.props {
                            // Find if there's a template that needs these imports
                            for check_template in &found_item.templates {
                                if let Some(import_template) = &check_template.per_prop_import {
                                    let import_line = import_template
                                        .replace("<prop>", &prop.entity_name)
                                        .replace("$prop$", &prop.entity_name)
                                        .replace("<path>", &env_vars.path)
                                        .replace("$path$", &env_vars.path)
                                        .replace("<dq>", &env_vars.dq)
                                        .replace("$dq$", &env_vars.dq);
                                    imports_to_add.push(import_line);
                                }
                            }
                        }
                    }
                }

                // Second pass: generate files
                for template_config in &found_item.templates {
                    let template_path = format!("{}{}", config.templates_root, template_config.template);
                    
                    println!("\n{} {}", lang_strings["template.reading"], template_path);

                    // Read template content
                    let template_content = match fs::read_to_string(&template_path) {
                        Ok(content) => content,
                        Err(e) => {
                            eprintln!("{} {} - {}", lang_strings["template.error_read"], template_path, e);
                            continue;
                        }
                    };

                    // Check if this template needs imports
                    let needs_imports = template_config.per_prop_import.is_some();

                    // Handle per_prop templates (generate one file per property)
                    if template_config.per_prop == Some(true) {
                        println!("{} {}", lang_strings["template.per_prop_gen"], template_config.destination);
                        
                        for prop in &env_vars.props {
                            // Replace property-specific variables in destination
                            let dest = template_config.destination
                                .replace("<prop>", &prop.entity_name)
                                .replace("$prop$", &prop.entity_name)
                                .replace("<Ent>", &env_vars.entity_name)
                                .replace("$Ent$", &env_vars.entity_name)
                                .replace("<path>", &env_vars.path)
                                .replace("$path$", &env_vars.path);

                            let final_destination = format!("{}{}", config.current_dir, dest);

                            // Create env_vars with single prop context
                            let mut single_prop_env = env_vars.clone();
                            single_prop_env.props = vec![prop.clone()];

                            // Apply template replacer with single prop context
                            let mut processed = template_replacer(&template_content, single_prop_env);
                            
                            // Replace specific prop variables (since we have only one prop now)
                            processed = processed
                                .replace("$ent_prop$", &prop.entity_name)
                                .replace("<ent_prop>", &prop.entity_name)
                                .replace("$prop$", &prop.name)
                                .replace("<prop>", &prop.name)
                                .replace("$prop_type$", &prop.prop_type)
                                .replace("<prop_type>", &prop.prop_type)
                                .replace("$camel_prop$", &prop.camel_name)
                                .replace("<camel_prop>", &prop.camel_name)
                                .replace("$snake_prop$", &prop.snake_name)
                                .replace("<snake_prop>", &prop.snake_name)
                                .replace("$kebab_prop$", &prop.kebab_name)
                                .replace("<kebab_prop>", &prop.kebab_name)
                                .replace("$const_prop$", &prop.constant_name)
                                .replace("<const_prop>", &prop.constant_name);

                            // Write file
                            write_file(&final_destination, &processed, lang_strings);
                        }
                    } else {
                        // Regular template (one file)
                        println!("{} {}", lang_strings["template.processing"], template_config.destination);

                        let dest = template_config.destination
                            .replace("<Ent>", &env_vars.entity_name)
                            .replace("$Ent$", &env_vars.entity_name)
                            .replace("<ent>", &env_vars.entity_name)
                            .replace("$ent$", &env_vars.entity_name)
                            .replace("<path>", &env_vars.path)
                            .replace("$path$", &env_vars.path);

                        let final_destination = format!("{}{}", config.current_dir, dest);

                        // Apply template replacer
                        let mut processed = template_replacer(&template_content, env_vars.clone());

                        // Add imports if this template needs them
                        if needs_imports && !imports_to_add.is_empty() {
                            let imports_block = imports_to_add.join("\n");
                            println!("{} {} imports", lang_strings["template.imports_added"], imports_to_add.len());
                            // Replace the placeholder
                            processed = processed.replace("$per_prop_imports$", &imports_block);
                            processed = processed.replace("<per_prop_imports>", &imports_block);
                        } else {
                            // Remove placeholder if no imports
                            processed = processed.replace("$per_prop_imports$", "");
                            processed = processed.replace("<per_prop_imports>", "");
                        }

                        // Write file
                        write_file(&final_destination, &processed, lang_strings);
                    }
                }

                println!("\n{} 🚀\n", lang_strings["template.completed"]);

            } else {
                println!("Option not found.");
                help_callback();
                process::exit(0x0100);
            }
        }
        Err(e) => {
            eprintln!("Failed to read Arq JSON: {}", e);
            panic!("");
        }
    }
}

fn write_file(file_path: &str, content: &str, lang: &std::collections::HashMap<&str, &str>) {
    let path = Path::new(file_path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            println!("{} {}", lang["template.creating_dir"], parent.display());
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("{} {} - {}", lang["template.error_mkdir"], parent.display(), e);
                return;
            }
        }
    }

    // Check if file exists
    if path.exists() {
        println!("\n{} {}", lang["template.file_exists"], file_path);
        print!("{} ", lang["template.overwrite_prompt"]);
        if let Err(_) = io::stdout().flush() {
            return;
        }

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            return;
        }
        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "s" | "y" | "yes" | "si" | "sí" => {
                // Continue to write
            }
            "a" | "abort" | "abortar" => {
                println!("{}", lang["template.aborting"]);
                process::exit(0);
            }
            _ => {
                println!("{} {}", lang["template.skipping"], file_path);
                return;
            }
        }
    }

    // Write file
    println!("{} {}", lang["template.writing_file"], file_path);
    if let Err(e) = fs::write(file_path, content) {
        eprintln!("{} {} - {}", lang["template.error_write"], file_path, e);
        return;
    }

    println!("{} {}", lang["template.success"], file_path);
}
