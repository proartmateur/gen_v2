pub fn env_vars_doc() -> String {
    let text: &'static str = "======= Template Env Vars =======
    $raw_name$ or <raw_name> : Name writed in command line
    $ent$ or <ent> : Name as PascalCase
    $camel_name$ or <camel_name> : Name as CamelCase
    $snake_name$ or <snake_name> : Name as snake_case
    $kebab_name$ or <kebab_name> : Name as kebab-case
    $const_name$ or <const_name> : Name as MAYUS_SNAKE_CASE
    $inline_props$ or <inline_props> : Props as you writed in command line
    $pretty_props$ or <pretty_props> : Props with line breaks after colon ','
    $snake_separated_props$ or <snake_separated_props> : Coming soon
    $entity_separated_props$ or <entity_separated_props> : Coming soon
    $camel_separated_props$ or <camel_separated_props> : Coming soon
    $prop$ or <prop> : Coming soon
    $author_name$ or <author_name> : Who is coding. Can be edited inside the gen_config.json file
    $author_email$ or <author_email> : Email from author
    $now$ or <now> : UTC Time when code will be generated
    $path$ or <path> : Path of ArqItem, parent of template path. Useful for imports in templates.
    $dq$ or <dq> : Double quote, parsed as \"
    ";

    return text.to_string();
}