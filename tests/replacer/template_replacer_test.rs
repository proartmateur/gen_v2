extern crate gen;

use gen::{
    arq::{arq_item::ArqItem, arq_template::ArqTemplate}, config::domain::config::Config, environment::env_mapper::env_mapper, replacer::template_replacer::template_replacer
};

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
fn single_template_only_entity() {
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let raw_template: &'static str = "/********************************
    Author: <author_name> <$author_email$>
    Description: Simple template
********************************/
    
    class $ent$ {
    }

    ";
    let template = raw_template.to_string();
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
    let env_vars = env_mapper(&arq_item, &name, &config, None);
    //#region

    //When
    let result = template_replacer(&template, env_vars);

    //#region Then
    let raw_expected: &'static str = "/********************************
    Author: myself <myself@example.com>
    Description: Simple template
********************************/
    
    class MartilloThor {
    }

    ";
    let expected = raw_expected.to_string();
    assert_eq!(result, expected);
    //#region
}


#[test]
fn single_template_only_entity_multiple_vars() {
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let raw_template: &'static str = "/********************************
    Author: <author_name> <$author_email$>
    Description: Simple template
********************************/
    
    class $ent$ {
        $pretty_props$
        constructor(<inline_props>){}
    }

    let <snake_name> = 0;
    let $camel_name$ = 1;
    let json = { <dq>$kebab_name$<dq>:<dq>$path$<dq> }

    ";
    let template = raw_template.to_string();
    let name = String::from("maRtillo_Thor");
    let props = String::from("name:str,age:int");
    let arq_item = ArqItem {
        name: "raw".to_string(),
        path: "/src/raw".to_string(),
        short_option: "-r".to_string(),
        option: "--raw".to_string(),
        description: "Code with props".to_string(),
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
    let env_vars = env_mapper(&arq_item, &name, &config, Some(props));
    //#region

    //When
    let result = template_replacer(&template, env_vars);

    //#region Then
    let raw_expected: &'static str = "/********************************
    Author: myself <myself@example.com>
    Description: Simple template
********************************/
    
    class MartilloThor {
        name:str,\nage:int
        constructor(name:str,age:int){}
    }

    let martillo_thor = 0;
    let martilloThor = 1;
    let json = { \"martillo-thor\":\"/src/raw\" }

    ";
    let expected = raw_expected.to_string();
    assert_eq!(result, expected);
    //#region
}


#[test]
fn template_prop_vars() {
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let raw_template: &'static str = "
    class $ent$ {
(        $ent_prop$;\n)
        constructor(

        ){}
    }

    ";
    let template = raw_template.to_string();
    let name = String::from("maRtillo_Thor");
    let props = String::from("User_name_1: str,age: int");
    let arq_item = ArqItem {
        name: "raw".to_string(),
        path: "/src/raw".to_string(),
        short_option: "-r".to_string(),
        option: "--raw".to_string(),
        description: "Code with props".to_string(),
        templates: vec![ArqTemplate {
            template: "/raw/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: Some(2),
        prop_prop_place: Some(1),
        prop_prefix: None,
        prop_type_separator: Some(String::from(": ")),
    };
    let config = get_config();
    let env_vars = env_mapper(&arq_item, &name, &config, Some(props));
    //#region

    //When
    let result = template_replacer(&template, env_vars);

    //#region Then
    let raw_expected: &'static str = "
    class MartilloThor {
       UserName1;
       Age;

        constructor(

        ){}
    }

    ";
    let expected = raw_expected.to_string();
    assert_eq!(result, expected);
    //#region
}


#[test]
fn template_prop_vars_with_types() {
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let raw_template: &'static str = "
    class $ent$ {
(        $ent_prop$<type_separator>$prop_type$;\n)
        constructor(

        ){}
    }

    ";
    let template = raw_template.to_string();
    let name = String::from("maRtillo_Thor");
    let props = String::from("User_name_1: str,age: int");
    let arq_item = ArqItem {
        name: "raw".to_string(),
        path: "/src/raw".to_string(),
        short_option: "-r".to_string(),
        option: "--raw".to_string(),
        description: "Code with props".to_string(),
        templates: vec![ArqTemplate {
            template: "/raw/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: Some(2),
        prop_prop_place: Some(1),
        prop_prefix: None,
        prop_type_separator: Some(String::from(": ")),
    };
    let config = get_config();
    let env_vars = env_mapper(&arq_item, &name, &config, Some(props));
    //#region

    //When
    let result = template_replacer(&template, env_vars);

    //#region Then
    let raw_expected: &'static str = "
    class MartilloThor {
       UserName1: str;
       Age: int;

        constructor(

        ){}
    }

    ";
    let expected = raw_expected.to_string();
    assert_eq!(result, expected);
    //#region
}


#[test]
fn template_prop_vars_with_types_php_style() {
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let raw_template: &'static str = "
    class $ent$ {
(        $$snake_prop$<type_separator>$prop_type$;\n)
        constructor(

        ){}
    }

    ";
    let template = raw_template.to_string();
    let name = String::from("maRtillo_Thor");
    let props = String::from("User_name_1: str,age: int");
    let arq_item = ArqItem {
        name: "raw".to_string(),
        path: "/src/raw".to_string(),
        short_option: "-r".to_string(),
        option: "--raw".to_string(),
        description: "Code with props".to_string(),
        templates: vec![ArqTemplate {
            template: "/raw/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: Some(2),
        prop_prop_place: Some(1),
        prop_prefix: None,
        prop_type_separator: Some(String::from(": ")),
    };
    let config = get_config();
    let env_vars = env_mapper(&arq_item, &name, &config, Some(props));
    //#region

    //When
    let result = template_replacer(&template, env_vars);

    //#region Then
    let raw_expected: &'static str = "
    class MartilloThor {
       $user_name_1: str;
       $age: int;

        constructor(

        ){}
    }

    ";
    let expected = raw_expected.to_string();
    assert_eq!(result, expected);
    //#region
}


#[test]
fn template_multiple_prop_vars_flavors_with_types() {
    //#region GIVEN
    // User write a call to gen in the terminal
    // CLI example:
    // gen --raw maRtillo_Thor
    // AND
    // In arq.json I have an ArqItem for option --raw
    // AND
    // I have a Config
    let raw_template: &'static str = "
( <const_prop> as $prop_type$;\n)
( // $kebab_prop$-> $prop_type$;\n)
( // $camel_prop$-> $prop_type$;\n)
    class $ent$ {
(        $ent_prop$<type_separator>$prop_type$;\n)
        constructor($ln$(           public <snake_prop>,\n)
         ){}
    }

    ";
    let template = raw_template.to_string();
    let name = String::from("maRtillo_Thor");
    let props = String::from("User_name_1: str,age: int");
    let arq_item = ArqItem {
        name: "raw".to_string(),
        path: "/src/raw".to_string(),
        short_option: "-r".to_string(),
        option: "--raw".to_string(),
        description: "Code with props".to_string(),
        templates: vec![ArqTemplate {
            template: "/raw/component.jsx".to_string(),
            destination: "<path>/<ent>.jsx".to_string(),
            per_prop: None,
            prop_naming: None,
            per_prop_import: None,
        }],
        has_props: Some(true),
        prop_type_place: Some(2),
        prop_prop_place: Some(1),
        prop_prefix: None,
        prop_type_separator: Some(String::from(": ")),
    };
    let config = get_config();
    let env_vars = env_mapper(&arq_item, &name, &config, Some(props));
    //#region

    //When
    let result = template_replacer(&template, env_vars);

    //#region Then
    let raw_expected: &'static str = "
USER_NAME_1 as str;
AGE as int;

// user-name-1-> str;
// age-> int;

// userName1-> str;
// age-> int;

    class MartilloThor {
       UserName1: str;
       Age: int;

        constructor(\n          public user_name_1,
          public age,

         ){}
    }

    ";
    let expected = raw_expected.to_string();
    assert_eq!(result, expected);
    //#region
}
