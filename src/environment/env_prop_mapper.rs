use crate::utils::string_transform::{snake_to_camel_case, snake_to_pascal_case};
use super::{prop_style::PropStyle, prop_vars::PropVars};

pub fn env_prop_mapper(data: &String, style: &PropStyle) -> Vec<PropVars> {
    let space: &str = &style.prefix_space.clone().unwrap_or("*".to_string());
    let mut result: Vec<PropVars> = Vec::new();
    let prop_split: Vec<&str> = data.split(",").collect();

    for item in prop_split {
        // prefix separation
        let mut prefix_part = String::new();
        let mut prop_full_part = String::new();
        let prop_full_part_clean: String;
        let mut prop_name = String::new();
        let mut prop_type = String::new();

        match &style.prefix {
            Some(prefix) => {
                let p: Vec<&str> = item.split(prefix).collect();
                if p.len() > 1 {
                    prefix_part = prefix.to_string();
                    prop_full_part = p[1].to_string();
                }
            }
            None => {
                prop_full_part = item.to_string();
            }
        }

        //check  if have type
        match &style.type_separator {
            Some(s) => {
                prop_full_part_clean =  prop_full_part.replace(space, " ").to_string();
                let separa = s.to_string();
                let pst: Vec<&str> = prop_full_part_clean.split(&separa).collect();
                let p_index: usize = &style.prop_place - 1;
                let t_index: usize = &style.type_place - 1;
                prop_name = pst[p_index].to_string();
                prop_type = pst[t_index].to_string();
            }
            None => {
                prop_name = prop_full_part;
            }
        }

        // FIll vars
        let prop_vars: PropVars = PropVars {
            name: format!("{}", prop_name),
            prop_type: format!("{}", prop_type),
            prefix: format!("{}", prefix_part.replace(space, " ")),
            entity_name: snake_to_pascal_case(&prop_name),
            snake_name: format!("{}", &prop_name.to_lowercase()),
            camel_name: format!("{}", snake_to_camel_case(&prop_name)),
            kebab_name: format!("{}", &prop_name.to_lowercase().replace("_", "-")),
            constant_name: format!("{}", &prop_name.to_ascii_uppercase()),
        };
        result.push(prop_vars);
    }

    return result;
}
