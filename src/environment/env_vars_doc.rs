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

Prop template Syntax:

  ($prop$) or (<prop>) : Write prop name for each prop

  ($prop$, ) or (<prop>, ) : Write 'prop_name, ' for each prop

  Available prop flavors:
   - prop: raw name of prop
   - ent_prop: PascalCase prop
   - camel_prop: camelCase prop
   - snake_prop: snake_name of prop
   - kebab_prop: kebab-named prop
   - const_prop: CONST_NAME of prop


$author_name$ or <author_name> : Who is coding. Can be edited inside the gen_config.json file

$author_email$ or <author_email> : Email from author

$now$ or <now> : UTC Time when code will be generated

$path$ or <path> : Path of ArqItem, parent of template path. Useful for imports in templates.

$dq$ or <dq> : Double quote, parsed as \"

$ln$ or <ln> : line break \\n

    ";

    return text.to_string();
}