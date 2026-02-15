use regex::Regex;
use crate::environment::{env_vars::EnvVars, prop_vars::PropVars};

/// Strips the leading `(` (+ one space) and trailing `)` delimiters from the captured block,
/// then replaces ALL prop variables for a single prop, preserving any internal `)` in the content.
fn clean_prop_value(prop_template: &str, prop: &PropVars) -> String {
    // Strip the outer ( and ) delimiters
    let mut inner = prop_template.to_string();
    if let Some(start) = inner.find('(') {
        let after = start + 1;
        if after < inner.len() && inner.as_bytes()[after] == b' ' {
            inner = format!("{}{}", &inner[..start], &inner[after + 1..]);
        } else {
            inner = format!("{}{}", &inner[..start], &inner[after..]);
        }
    }
    if let Some(end) = inner.rfind(')') {
        inner = format!("{}{}", &inner[..end], &inner[end + 1..]);
    }

    let type_sep = prop.type_separator.clone().unwrap_or_default();
    let result = inner
        // Replace ALL prop name variants (most specific first)
        .replace("$ent_prop$", &prop.entity_name)
        .replace("<ent_prop>", &prop.entity_name)
        .replace("$snake_prop$", &prop.snake_name)
        .replace("<snake_prop>", &prop.snake_name)
        .replace("$camel_prop$", &prop.camel_name)
        .replace("<camel_prop>", &prop.camel_name)
        .replace("$kebab_prop$", &prop.kebab_name)
        .replace("<kebab_prop>", &prop.kebab_name)
        .replace("$const_prop$", &prop.constant_name)
        .replace("<const_prop>", &prop.constant_name)
        .replace("$prop$", &prop.name)
        .replace("<prop>", &prop.name)
        // Replace type and separator
        .replace("<type_separator>", &type_sep)
        .replace("$type_separator$", &type_sep)
        .replace("$prop_type$", &prop.prop_type)
        .replace("<prop_type>", &prop.prop_type);

    return result;
}

fn process_prop_data(prop_template: &str, vars: &EnvVars) -> String {
    let mut res = String::new();
    for prop in vars.props.iter() {
        let value = clean_prop_value(prop_template, prop);
        res.push_str(&value);
    }
    return res;
}

fn replace_props(template: &String, vars: &EnvVars, re: &Regex) -> String {
    let mut res = String::from(template);

    // Collect all matches first - use capture group 1 which is just the (...)  block
    let matches: Vec<String> = re
        .captures_iter(&template)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect();

    // Replace each match with the expanded prop data
    for match_str in matches {
        let pp = process_prop_data(&match_str, vars);
        res = res.replacen(&match_str, &pp, 1);
    }

    return res;
}

pub fn template_replacer(template: &String, vars: EnvVars) -> String {
    let mut result = String::from(template);

    // First resolve $ln$ and \n so that multi-line blocks are properly formed
    // before the iterative regex runs.
    result = result.replace("<ln>", "\n").replace("$ln$", "\n");

    // (?s) enables dot-all mode so `.` also matches newlines.
    // (?:^|\n) ensures `(` is at the start of a line (not inside code like `constructor(`)
    // The closing `)` must be at the start of a line (with optional whitespace)
    // to distinguish from code parentheses like `getValue()`.
    let prop_regex = Regex::new(
        r"(?sm)(?:^|\n)(\((?:[^\)]*?(?:\$(?:ent_prop|snake_prop|camel_prop|kebab_prop|const_prop|prop)\$|<(?:ent_prop|snake_prop|camel_prop|kebab_prop|const_prop|prop)>)).*?\n\))"
    ).unwrap();

    if vars.props.len() > 0 {
        // Keep iterating until no more matches (handles nested patterns)
        loop {
            let t = replace_props(&result, &vars, &prop_regex);
            if t == result {
                break;
            }
            result = t;
        }
    }

    result = result
        .replace("<raw_name>", &vars.author_email.clone().unwrap_or_default())
        .replace("$raw_name$", &vars.author_email.clone().unwrap_or_default())
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
        .replace("$ln$", "\n")
        .replace(r"\n", "\n")
        .replace("\\n", "\n");

    // Generate FILE_HEADER if needed
    let file_header = format!(
        "/**\n * @author {} <{}>\n * @date {}\n */",
        &vars.author_name.clone().unwrap_or_default(),
        &vars.author_email.clone().unwrap_or_default(),
        &vars.now.clone().unwrap_or_default()
    );
    result = result
        .replace("<FILE_HEADER>", &file_header)
        .replace("$FILE_HEADER$", &file_header);

    return String::from(result);
}
