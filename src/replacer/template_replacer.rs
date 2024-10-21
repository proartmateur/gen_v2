use crate::environment::env_vars::EnvVars;

pub fn template_replacer(template: &String, vars: EnvVars) -> String {
    let result = template
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
        .replace("$dq$", &vars.dq.clone());
    return String::from(result);
}
