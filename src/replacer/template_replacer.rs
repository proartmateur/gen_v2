use regex::Regex;
use crate::environment::{env_vars::EnvVars, prop_vars::PropVars};

fn clean_prop_value(prop_template: &str, val:&str, var_name: &str, prop: &PropVars) -> String {
    let var_name_a = format!(r#"${}$"#,var_name);
    let var_name_b = format!(r#"<{}>"#,var_name);
    let result = prop_template
    .replace("<type_separator>", prop.type_separator.clone().unwrap().as_str())
    .replace("$type_separator$", prop.type_separator.clone().unwrap().as_str())
    .replace("$prop_type$", prop.prop_type.clone().as_str())
    .replace("<prop_type>", prop.prop_type.clone().as_str())
    .replace(var_name_a.as_str(), val)
    .replace(var_name_b.as_str(), val)
    .replace(r#"( "#,"")
    .replace(r#")"#,"");
    
    return result;
}
fn get_prop_value(prop_template: &str, prop: &PropVars) -> String {
    let mut res: String = String::from("");
    
    if prop_template.contains("ent_prop") {
        res = clean_prop_value(&prop_template, &prop.entity_name,"ent_prop", prop);
    } else if prop_template.contains("snake_prop") {
        res = clean_prop_value(&prop_template, &prop.snake_name,"snake_prop", prop);
    } else if prop_template.contains("kebab_prop") {
        res = clean_prop_value(&prop_template, &prop.kebab_name,"kebab_prop", prop);
    } else if prop_template.contains("const_prop") {
        res = clean_prop_value(&prop_template, &prop.constant_name,"const_prop", prop);
    } else if prop_template.contains("camel_prop") {
        res = clean_prop_value(&prop_template, &prop.camel_name,"camel_prop", prop);
    } else if prop_template.contains("prop") {
        res = clean_prop_value(&prop_template, &prop.name,"prop", prop);
    } 

    return res;
}

fn process_prop_data(prop_template: &str, vars: &EnvVars)->String {
    
    let mut res = String::from("");

    for prop in vars.props.iter() {
        let value = get_prop_value(prop_template, prop);
        
        res = format!("{}{}",res,value);
        
    }

    
    return res;
}
fn replace_props(template: &String, vars: &EnvVars,re: &Regex) -> String {
    let mut res = String::from(template);
    let caps = re.captures(&template.as_str());
    if caps.is_none() {
        print!("No hay uso custom de props");
    } else {
        let cu = caps.unwrap();
        
        let total_cap = cu.len();
        if total_cap > 0 {
            let fc = cu.get(0).unwrap().as_str();
            let pp = process_prop_data(fc, vars);
            
            res = format!("{}",res.replace(fc, &pp));
        }
    }

    return res;
}

pub fn template_replacer(template: &String, vars: EnvVars) -> String {
    let mut result = String::from(template);
    
    let res: [Regex;6] = [ 
        Regex::new(r"\(\s *.*?(\$prop\$|<prop>).*?\s*\)").unwrap(),
        Regex::new(r"\(\s *.*?(\$ent_prop\$|<ent_prop>).*?\s*\)").unwrap(),
        Regex::new(r"\(\s *.*?(\$snake_prop\$|<snake_prop>).*?\s*\)").unwrap(),
        Regex::new(r"\(\s *.*?(\$camel_prop\$|<camel_prop>).*?\s*\)").unwrap(),
        Regex::new(r"\(\s *.*?(\$kebab_prop\$|<kebab_prop>).*?\s*\)").unwrap(),
        Regex::new(r"\(\s *.*?(\$const_prop\$|<const_prop>).*?\s*\)").unwrap(),
     ];

    
    for reg in res.iter() {
        if vars.props.len() > 0 {
            let t = replace_props(&result, &vars, &reg);
            result = format!("{}",t);
        }
    }

    
    
    result = result
        .replace("<raw_name>", &vars.author_email.clone().unwrap_or("".to_string()))
        .replace("$raw_name$", &vars.author_email.clone().unwrap_or("".to_string()))
        .replace("<ent>", &vars.entity_name)
        .replace("$ent$", &vars.entity_name)
        .replace("<camel_name>", &vars.camel_name)
        .replace("$camel_name$", &vars.camel_name)
        .replace("<snake_name>", &vars.snake_name)
        .replace("$snake_name$", &vars.snake_name)
        .replace("<kebab_name>", &vars.kebab_name)
        .replace("$kebab_name$", &vars.kebab_name)
        .replace("<const_name>", &vars.const_name)
        .replace("$const_name$", &vars.const_name)
        .replace("<inline_props>", &vars.inline_props.clone().unwrap_or_default())
        .replace("$inline_props$", &vars.inline_props.clone().unwrap_or_default())
        .replace("<pretty_props>", &vars.pretty_props.clone().unwrap_or_default())
        .replace("$pretty_props$", &vars.pretty_props.clone().unwrap_or_default())
        .replace("<author_name>", &vars.author_name.clone().unwrap_or_default())
        .replace("$author_name$", &vars.author_name.clone().unwrap_or_default())
        .replace("<author_email>", &vars.author_email.clone().unwrap_or_default())
        .replace("$author_email$", &vars.author_email.clone().unwrap_or_default())
        .replace("<now>", &vars.now.clone().unwrap_or_default())
        .replace("$now$", &vars.now.clone().unwrap_or_default())
        .replace("<path>", &vars.path.clone())
        .replace("$path$", &vars.path.clone())
        .replace("<dq>", &vars.dq.clone())
        .replace("$dq$", &vars.dq.clone())
        .replace("<ln>", "\n")
        .replace("$ln$", "\n");
    
    return String::from(result);
}
