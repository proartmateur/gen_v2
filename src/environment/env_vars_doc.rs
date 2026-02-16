pub fn env_vars_doc() -> String {
    let text: &'static str = r#"======= GenCLI Template Variables Reference =======

=== ENTITY VARIABLES ===
These variables are based on the entity name you provide in the command.

$raw_name$  or  <raw_name>   : Name as written in command line
$ent$       or  <ent>         : Name as PascalCase (e.g., UserAccount)
$camel_name$ or <camel_name>  : Name as camelCase (e.g., userAccount)
$snake_name$ or <snake_name>  : Name as snake_case (e.g., user_account)
$kebab_name$ or <kebab_name>  : Name as kebab-case (e.g., user-account)
$const_name$ or <const_name>  : Name as CONST_CASE (e.g., USER_ACCOUNT)

=== PROPERTY VARIABLES (Simple Replacement) ===
These are replaced when used outside of iterative blocks.

$inline_props$  or  <inline_props>  : Props as written in command line
$pretty_props$  or  <pretty_props>  : Props with line breaks after comma ','

=== ITERATIVE PROP BLOCKS ===
Code between (...) is repeated once per property.

Syntax rules:
  1. Opening '(' must be at the start of a line
  2. Closing ')' must be on its own line with a newline before it
  3. Must contain at least one prop variable

Example:
  (    private $camel_prop$: $prop_type$;
  )

  With props "name:string,age:number" becomes:
      private name: string;
      private age: number;

Available prop variable flavors:
  $prop$       or  <prop>        : Property name (original)
  $ent_prop$   or  <ent_prop>    : Property as PascalCase (e.g., FirstName)
  $camel_prop$ or  <camel_prop>  : Property as camelCase (e.g., firstName)
  $snake_prop$ or  <snake_prop>  : Property as snake_case (e.g., first_name)
  $kebab_prop$ or  <kebab_prop>  : Property as kebab-case (e.g., first-name)
  $const_prop$ or  <const_prop>  : Property as CONST_CASE (e.g., FIRST_NAME)

  $prop_type$      or  <prop_type>      : Property type (e.g., string, number)
  $type_separator$ or  <type_separator> : Type separator (e.g., ':' for TS, ' ' for Go)

=== SPECIAL VARIABLES ===

$FILE_HEADER$      : Auto-generated file header with author, email, and timestamp
                     Example output:
                     /**
                      * @author myself <myself@example.com>
                      * @date 2026-02-15 10:30:00.123456789 UTC
                      */

$per_prop_imports$ : Placeholder for imports collected from per_prop templates.
                     Used in templates that reference Value Objects or other
                     per-property generated classes.
                     Example:
                     import { NameVO } from "./value_objects/NameVO"
                     import { EmailVO } from "./value_objects/EmailVO"

=== METADATA VARIABLES ===

$author_name$  or  <author_name>  : Author name from gen_config.json
$author_email$ or  <author_email> : Author email from gen_config.json
$now$          or  <now>          : UTC timestamp when code is generated

=== PATH VARIABLES ===

$path$  or  <path>  : Resolved path from architecture config.
                      Useful for imports and relative references.
                      Example: If path="/src/<ent>" and entity="User",
                      then $path$ = "/src/User"

=== UTILITY VARIABLES ===

$dq$  or  <dq>  : Double quote character "
                  Use in arq.json for imports: $dq$./path$dq$ → "./path"

$ln$  or  <ln>  : Line break (newline character \n)

=== EXAMPLES ===

1. Simple variable replacement:
   Template:  export class $ent$ { }
   Command:   gen -c User
   Result:    export class User { }

2. Iterative prop block:
   Template:  (    private _$camel_prop$: $prop_type$;
              )
   Command:   gen -c User "name:string,age:number"
   Result:        private _name: string;
                  private _age: number;

3. Mixed variables in iterative block:
   Template:  (    private _$camel_prop$: $ent_prop$VO;
              )
   Command:   gen -c User "email:string"
   Result:        private _email: EmailVO;

4. Using per_prop_imports:
   Template:  $per_prop_imports$
              export class $ent$ {
              (    private _$camel_prop$: $ent_prop$VO;
              )
              }
   Result:    import { EmailVO } from "/src/User/value_objects/EmailVO"
              export class User {
                  private _email: EmailVO;
              }

For more information, see templates.md and arq_json_guide.md
"#;

    return text.to_string();
}