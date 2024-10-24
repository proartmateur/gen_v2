use gen::{
    arq::{arq_item::ArqItem, arq_template::ArqTemplate},
    config::domain::config::Config,
    environment::{env_mapper::env_mapper, env_vars::EnvVars, prop_vars::PropVars},
};

extern crate gen;

fn get_config() -> Config {
    let config = Config {
        author: Some("myself".to_string()),
        author_email: Some("myself@example.com".to_string()),
        templates_root: "/Users/my_folder/project/.gen_cli/templates".to_string(),
        current_dir: "/Users/my_folder/project".to_string(),
        arq_file: "arq.json".to_string(),
    };
    return config;
}

#[test]
fn test_single_boilerplate_no_props() {
    
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let name = String::from("maRtillo_Thor");
    let arq_item = ArqItem {
        name: "raw".to_string(),
        path: "/src/raw".to_string(),
        short_option: "-r".to_string(),
        option: "--raw".to_string(),
        description: "Code without props".to_string(),
        templates: vec![ArqTemplate {
            template: "/raw/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(false),
        prop_type_place: None,
        prop_prop_place: None,
        prop_prefix: None,
        prop_type_separator: None,
    };
    let config = get_config();
    //#region
    

    // WHEN
    let result: EnvVars = env_mapper(&arq_item, &name, &config, None);

    //#region THEN

    let expected = EnvVars {
        raw_name: "maRtillo_Thor".to_string(),
        entity_name: "MartilloThor".to_string(),
        camel_name: "martilloThor".to_string(),
        snake_name: "martillo_thor".to_string(),
        kebab_name: "martillo-thor".to_string(),
        const_name: "MARTILLO_THOR".to_string(),
        inline_props: None,
        pretty_props: None,
        props: vec![],
        author_name: Some(
            "myself".to_string(),
        ),
        author_email: Some(
            "myself@example.com".to_string(),
        ),
        now: Some(
            "2024-10-17 05:53:04.838087 UTC".to_string(),
        ),
        path: "/src/raw".to_string(),
        dq: "\"".to_string(),
    };

    //#region Assertions
    assert_eq!(result.raw_name, expected.raw_name);
    assert_eq!(result.entity_name, expected.entity_name);
    assert_eq!(result.camel_name, expected.camel_name);
    assert_eq!(result.snake_name, expected.snake_name);
    assert_eq!(result.kebab_name, expected.kebab_name);
    assert_eq!(result.const_name, expected.const_name);
    assert_eq!(result.inline_props, expected.inline_props);
    assert_eq!(result.pretty_props, expected.pretty_props);
    assert_eq!(result.props.len(), expected.props.len());
    assert_eq!(result.author_name, expected.author_name);
    assert_eq!(result.author_email, expected.author_email);
    assert_eq!(result.path, expected.path);
    assert_eq!(result.dq, expected.dq);
    //#region
    
    //#region
}


#[test]
fn test_boilerplate_props_without_types(){

    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --atom tHe_aToM name_aTom,age
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let name = String::from("tHe_aToM");
    let props = Some(String::from("name_aTom,age"));
    let arq_item = ArqItem {
        name: "atom".to_string(),
        path: "/src/atoms".to_string(),
        short_option: "-a".to_string(),
        option: "--atom".to_string(),
        description: "frontend atom component".to_string(),
        templates: vec![ArqTemplate {
            template: "/atom/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: None,
        prop_prop_place: None,
        prop_prefix: None,
        prop_type_separator: None,
    };
    let config = get_config();
    //#region

    // WHEN
    let result: EnvVars = env_mapper(&arq_item, &name, &config, props);

       //#region THEN

    let expected = EnvVars {
        raw_name: "tHe_aToM".to_string(),
        entity_name: "TheAtom".to_string(),
        camel_name: "theAtom".to_string(),
        snake_name: "the_atom".to_string(),
        kebab_name: "the-atom".to_string(),
        const_name: "THE_ATOM".to_string(),
        inline_props: Some("name_aTom,age".to_string()),
        pretty_props: Some(String::from("name_aTom,\nage")),
        props: vec![
            PropVars  {
                name: "name_aTom".to_string(),
                prop_type: "".to_string(),
                prefix: "".to_string(),
                entity_name: "NameAtom".to_string(),
                snake_name: "name_atom".to_string(),
                camel_name: "nameAtom".to_string(),
                kebab_name: "name-atom".to_string(),
                constant_name: "NAME_ATOM".to_string(),
                type_separator: None
            },
            PropVars  {
                name: "age".to_string(),
                prop_type: "".to_string(),
                prefix: "".to_string(),
                entity_name: "Age".to_string(),
                snake_name: "age".to_string(),
                camel_name: "age".to_string(),
                kebab_name: "age".to_string(),
                constant_name: "AGE".to_string(),
                type_separator: None
            },
        ],
        author_name: Some(
            "myself".to_string(),
        ),
        author_email: Some(
            "myself@example.com".to_string(),
        ),
        now: Some(
            "2024-10-17 05:53:04.838087 UTC".to_string(),
        ),
        path: "/src/atoms".to_string(),
        dq: "\"".to_string(),
    };

    //#region Assertions
    assert_eq!(result.raw_name, expected.raw_name);
    assert_eq!(result.entity_name, expected.entity_name);
    assert_eq!(result.camel_name, expected.camel_name);
    assert_eq!(result.snake_name, expected.snake_name);
    assert_eq!(result.kebab_name, expected.kebab_name);
    assert_eq!(result.const_name, expected.const_name);
    assert_eq!(result.inline_props, expected.inline_props);
    assert_eq!(result.pretty_props, expected.pretty_props);
    assert_eq!(result.props.len(), expected.props.len());
    assert_eq!(result.author_name, expected.author_name);
    assert_eq!(result.author_email, expected.author_email);
    assert_eq!(result.path, expected.path);
    assert_eq!(result.dq, expected.dq);
    //#region
    
    //#region
}


#[test]
fn test_boilerplate_typed_props(){

    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --atom tHe_aToM name_aTom:string,age:number
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let name = String::from("tHe_aToM");
    let props = Some(String::from("name_aTom:string,age:number"));
    let arq_item = ArqItem {
        name: "atom".to_string(),
        path: "/src/atoms".to_string(),
        short_option: "-a".to_string(),
        option: "--atom".to_string(),
        description: "frontend atom component".to_string(),
        templates: vec![ArqTemplate {
            template: "/atom/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: Some(1),
        prop_prop_place: Some(2),
        prop_prefix: None,
        prop_type_separator: Some(':'.to_string()),
    };

    let config = get_config();
    //#region

    // WHEN
    let result: EnvVars = env_mapper(&arq_item, &name, &config, props);

    //#region THEN

    let expected = EnvVars {
        raw_name: "tHe_aToM".to_string(),
        entity_name: "TheAtom".to_string(),
        camel_name: "theAtom".to_string(),
        snake_name: "the_atom".to_string(),
        kebab_name: "the-atom".to_string(),
        const_name: "THE_ATOM".to_string(),
        inline_props: Some("name_aTom:string,age:number".to_string()),
        pretty_props: Some(String::from("name_aTom:string,\nage:number")),
        props: vec![
            PropVars  {
                name: "name_aTom".to_string(),
                prop_type: "string".to_string(),
                prefix: "".to_string(),
                entity_name: "NameAtom".to_string(),
                snake_name: "name_atom".to_string(),
                camel_name: "nameAtom".to_string(),
                kebab_name: "name-atom".to_string(),
                constant_name: "NAME_ATOM".to_string(),
                type_separator: Some(':'.to_string()),
            },
            PropVars  {
                name: "age".to_string(),
                prop_type: "number".to_string(),
                prefix: "".to_string(),
                entity_name: "Age".to_string(),
                snake_name: "age".to_string(),
                camel_name: "age".to_string(),
                kebab_name: "age".to_string(),
                constant_name: "AGE".to_string(),
                type_separator: Some(':'.to_string()),
            },
        ],
        author_name: Some(
            "myself".to_string(),
        ),
        author_email: Some(
            "myself@example.com".to_string(),
        ),
        now: Some(
            "2024-10-17 05:53:04.838087 UTC".to_string(),
        ),
        path: "/src/atoms".to_string(),
        dq: "\"".to_string(),
    };

    //#region Assertions
    assert_eq!(result.raw_name, expected.raw_name);
    assert_eq!(result.entity_name, expected.entity_name);
    assert_eq!(result.camel_name, expected.camel_name);
    assert_eq!(result.snake_name, expected.snake_name);
    assert_eq!(result.kebab_name, expected.kebab_name);
    assert_eq!(result.const_name, expected.const_name);
    assert_eq!(result.inline_props, expected.inline_props);
    assert_eq!(result.pretty_props, expected.pretty_props);
    assert_eq!(result.props.len(), expected.props.len());
    assert_eq!(result.author_name, expected.author_name);
    assert_eq!(result.author_email, expected.author_email);
    assert_eq!(result.path, expected.path);
    assert_eq!(result.dq, expected.dq);
    //#region
    
    //#region
}

#[test]
fn test_boilerplate_typed_props_with_prefix(){

    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --atom tHe_aToM private*readonly*name_aTom:string,private*readonly*age:number
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let name = String::from("tHe_aToM");
    let props = Some(String::from("private*readonly*name_aTom:string,private*readonly*age:number"));
    let arq_item = ArqItem {
        name: "atom".to_string(),
        path: "/src/atoms".to_string(),
        short_option: "-a".to_string(),
        option: "--atom".to_string(),
        description: "frontend atom component".to_string(),
        templates: vec![ArqTemplate {
            template: "/atom/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: Some(1),
        prop_prop_place: Some(2),
        prop_prefix: Some(String::from("private*readonly*")),
        prop_type_separator: Some(':'.to_string()),
    };

    let config = get_config();
    //#region

    // WHEN
    let result: EnvVars = env_mapper(&arq_item, &name, &config, props);

    //#region THEN

    let expected = EnvVars {
        raw_name: "tHe_aToM".to_string(),
        entity_name: "TheAtom".to_string(),
        camel_name: "theAtom".to_string(),
        snake_name: "the_atom".to_string(),
        kebab_name: "the-atom".to_string(),
        const_name: "THE_ATOM".to_string(),
        inline_props: Some("private readonly name_aTom:string,private readonly age:number".to_string()),
        pretty_props: Some(String::from("private readonly name_aTom:string,\nprivate readonly age:number")),
        props: vec![
            PropVars  {
                name: "name_aTom".to_string(),
                prop_type: "string".to_string(),
                prefix: "private readonly ".to_string(),
                entity_name: "NameAtom".to_string(),
                snake_name: "name_atom".to_string(),
                camel_name: "nameAtom".to_string(),
                kebab_name: "name-atom".to_string(),
                constant_name: "NAME_ATOM".to_string(),
                type_separator: Some(String::from(":")),
            },
            PropVars  {
                name: "age".to_string(),
                prop_type: "number".to_string(),
                prefix: "private readonly ".to_string(),
                entity_name: "Age".to_string(),
                snake_name: "age".to_string(),
                camel_name: "age".to_string(),
                kebab_name: "age".to_string(),
                constant_name: "AGE".to_string(),
                type_separator: Some(String::from(":")),
            },
        ],
        author_name: Some(
            "myself".to_string(),
        ),
        author_email: Some(
            "myself@example.com".to_string(),
        ),
        now: Some(
            "2024-10-17 05:53:04.838087 UTC".to_string(),
        ),
        path: "/src/atoms".to_string(),
        dq: "\"".to_string(),
    };

    //#region Assertions
    assert_eq!(result.raw_name, expected.raw_name);
    assert_eq!(result.entity_name, expected.entity_name);
    assert_eq!(result.camel_name, expected.camel_name);
    assert_eq!(result.snake_name, expected.snake_name);
    assert_eq!(result.kebab_name, expected.kebab_name);
    assert_eq!(result.const_name, expected.const_name);
    assert_eq!(result.inline_props, expected.inline_props);
    assert_eq!(result.pretty_props, expected.pretty_props);
    assert_eq!(result.props.len(), expected.props.len());
    assert_eq!(result.author_name, expected.author_name);
    assert_eq!(result.author_email, expected.author_email);
    assert_eq!(result.path, expected.path);
    assert_eq!(result.dq, expected.dq);
    //#region
    
    //#region
}