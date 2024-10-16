
extern crate gen;
use gen::environment::prop_style::PropStyle;
use gen::environment::prop_vars::PropVars;
use gen::environment::env_prop_mapper::env_prop_mapper;
#[test]
fn test_typescript_style_props() {
    let props:String = "name:string,The_age:number".to_string();
    
    let style = PropStyle {
        type_separator: Some(':'),
        prop_place: 1,
        type_place: 2,
        prefix: None
    };

    let expectation_1 = PropVars {
        name: format!("{}","name"),
        prop_type: format!("{}","string"),
        prefix: format!("{}",""),
        entity_name: format!("{}","Name"),
        snake_name: format!("{}","name"),
        camel_name: format!("{}","name"),
        kebab_name: format!("{}","name"),
        constant_name: format!("{}","name".to_uppercase())
    };

    let expectation_2 = PropVars {
        name: format!("{}","The_age"),
        prop_type: format!("{}","number"),
        prefix: format!("{}",""),
        entity_name: format!("{}","TheAge"),
        snake_name: format!("{}","the_age"),
        camel_name: format!("{}","theAge"),
        kebab_name: format!("{}","the-age"),
        constant_name: format!("{}","the_age".to_uppercase()),
    };

    let mut expected_result: Vec<PropVars> = Vec::new();
    expected_result.push(expectation_1);
    expected_result.push(expectation_2);
    
    let result = env_prop_mapper(&props, &style);
    assert_eq!(result.len(),expected_result.len());
    assert_eq!(&result[0].name, &expected_result[0].name);
    assert_eq!(&result[0].prop_type, &expected_result[0].prop_type);
    assert_eq!(&result[0].entity_name, &expected_result[0].entity_name);
    assert_eq!(&result[0].prefix, &expected_result[0].prefix);
    assert_eq!(&result[1].entity_name, &expected_result[1].entity_name);
    assert_eq!(&result[1].snake_name, &expected_result[1].snake_name);
    assert_eq!(&result[1].camel_name, &expected_result[1].camel_name);
    assert_eq!(&result[1].constant_name, &expected_result[1].constant_name);
    assert_eq!(&result[1].prop_type, &expected_result[1].prop_type);
}


#[test]
fn test_java_style_props() {
    let props:String = "pub_name:string,pub_The_age:number".to_string();
    
    let style = PropStyle {
        type_separator: Some(':'),
        prop_place: 1,
        type_place: 2,
        prefix: Some(format!("{}","pub_"))
    };

    let expectation_1 = PropVars {
        name: format!("{}","name"),
        prop_type: format!("{}","string"),
        prefix: format!("{}","pub "),
        entity_name: format!("{}","Name"),
        snake_name: format!("{}","name"),
        camel_name: format!("{}","name"),
        kebab_name: format!("{}","name"),
        constant_name: format!("{}","name".to_uppercase()),
    };
    
    let expectation_2 = PropVars {
        name: format!("{}","The_age"),
        prop_type: format!("{}","number"),
        prefix: format!("{}","pub "),
        entity_name: format!("{}","TheAge"),
        snake_name: format!("{}","the_age"),
        camel_name: format!("{}","theAge"),
        kebab_name: format!("{}","the-age"),
        constant_name: format!("{}","the_age".to_uppercase()),
    };

    let mut expected_result: Vec<PropVars> = Vec::new();
    expected_result.push(expectation_1);
    expected_result.push(expectation_2);
    
    let result = env_prop_mapper(&props, &style);
    assert_eq!(result.len(),expected_result.len());
    assert_eq!(&result[0].name, &expected_result[0].name);
    assert_eq!(&result[0].prop_type, &expected_result[0].prop_type);
    assert_eq!(&result[0].entity_name, &expected_result[0].entity_name);
    assert_eq!(&result[0].prefix, &expected_result[0].prefix);
    assert_eq!(&result[1].entity_name, &expected_result[1].entity_name);
    assert_eq!(&result[1].snake_name, &expected_result[1].snake_name);
    assert_eq!(&result[1].camel_name, &expected_result[1].camel_name);
    assert_eq!(&result[1].constant_name, &expected_result[1].constant_name);
    assert_eq!(&result[1].prop_type, &expected_result[1].prop_type);
}


#[test]
fn test_php_without_types_nor_prefix_style_props() {
    let props:String = "name,The_age".to_string();
    
    let style = PropStyle {
        type_separator: None,
        prop_place: 0,
        type_place: 0,
        prefix: None
    };

    let expectation_1 = PropVars {
        name: format!("{}","name"),
        prop_type: format!("{}",""),
        prefix: format!("{}",""),
        entity_name: format!("{}","Name"),
        snake_name: format!("{}","name"),
        camel_name: format!("{}","name"),
        kebab_name: format!("{}","name"),
        constant_name: format!("{}","name".to_uppercase()),
    };

    let expectation_2 = PropVars {
        name: format!("{}","The_age"),
        prop_type: format!("{}",""),
        prefix: format!("{}",""),
        entity_name: format!("{}","TheAge"),
        snake_name: format!("{}","the_age"),
        camel_name: format!("{}","theAge"),
        kebab_name: format!("{}","the-age"),
        constant_name: format!("{}","the_age".to_uppercase()),
    };

    let mut expected_result: Vec<PropVars> = Vec::new();
    expected_result.push(expectation_1);
    expected_result.push(expectation_2);
    
    let result = env_prop_mapper(&props, &style);
    assert_eq!(result.len(),expected_result.len());
    assert_eq!(&result[0].name, &expected_result[0].name);
    assert_eq!(&result[0].prop_type, &expected_result[0].prop_type);
    assert_eq!(&result[0].entity_name, &expected_result[0].entity_name);
    assert_eq!(&result[0].prefix, &expected_result[0].prefix);
    assert_eq!(&result[1].entity_name, &expected_result[1].entity_name);
    assert_eq!(&result[1].snake_name, &expected_result[1].snake_name);
    assert_eq!(&result[1].camel_name, &expected_result[1].camel_name);
    assert_eq!(&result[1].constant_name, &expected_result[1].constant_name);
    assert_eq!(&result[1].prop_type, &expected_result[1].prop_type);
}

#[test]
fn test_php_without_types_style_props() {
    let props:String = "pub_name,pub_The_age".to_string();
    
    let style = PropStyle {
        type_separator: None,
        prop_place: 1,
        type_place: 2,
        prefix: Some(format!("{}","pub_"))
    };

    let expectation_1 = PropVars {
        name: format!("{}","name"),
        prop_type: format!("{}",""),
        prefix: format!("{}","pub "),
        entity_name: format!("{}","Name"),
        snake_name: format!("{}","name"),
        camel_name: format!("{}","name"),
        kebab_name: format!("{}","name"),
        constant_name: format!("{}","name".to_uppercase()),
    };

    let expectation_2 = PropVars {
        name: format!("{}","The_age"),
        prop_type: format!("{}",""),
        prefix: format!("{}","pub "),
        entity_name: format!("{}","TheAge"),
        snake_name: format!("{}","the_age"),
        camel_name: format!("{}","theAge"),
        kebab_name: format!("{}","the-age"),
        constant_name: format!("{}","the_age".to_uppercase()),
    };

    let mut expected_result: Vec<PropVars> = Vec::new();
    expected_result.push(expectation_1);
    expected_result.push(expectation_2);
    
    let result = env_prop_mapper(&props, &style);
    assert_eq!(result.len(),expected_result.len());
    assert_eq!(&result[0].name, &expected_result[0].name);
    assert_eq!(&result[0].prop_type, &expected_result[0].prop_type);
    assert_eq!(&result[0].entity_name, &expected_result[0].entity_name);
    assert_eq!(&result[0].prefix, &expected_result[0].prefix);
    assert_eq!(&result[1].entity_name, &expected_result[1].entity_name);
    assert_eq!(&result[1].snake_name, &expected_result[1].snake_name);
    assert_eq!(&result[1].camel_name, &expected_result[1].camel_name);
    assert_eq!(&result[1].constant_name, &expected_result[1].constant_name);
    assert_eq!(&result[1].prop_type, &expected_result[1].prop_type);
}


#[test]
fn test_go_style_props() {
    let props:String = "name*string,The_age*int".to_string();
    
    let style = PropStyle {
        type_separator: Some(' '),
        prop_place: 1,
        type_place: 2,
        prefix: None
    };

    let expectation_1 = PropVars {
        name: format!("{}","name"),
        prop_type: format!("{}","string"),
        prefix: format!("{}",""),
        entity_name: format!("{}","Name"),
        snake_name: format!("{}","name"),
        camel_name: format!("{}","name"),
        kebab_name: format!("{}","name"),
        constant_name: format!("{}","name".to_uppercase()),
    };

    let expectation_2 = PropVars {
        name: format!("{}","The_age"),
        prop_type: format!("{}","int"),
        prefix: format!("{}",""),
        entity_name: format!("{}","TheAge"),
        snake_name: format!("{}","the_age"),
        camel_name: format!("{}","theAge"),
        kebab_name: format!("{}","the-age"),
        constant_name: format!("{}","the_age".to_uppercase()),
    };

    let mut expected_result: Vec<PropVars> = Vec::new();
    expected_result.push(expectation_1);
    expected_result.push(expectation_2);
    
    let result = env_prop_mapper(&props, &style);
    assert_eq!(result.len(),expected_result.len());
    assert_eq!(&result[0].name, &expected_result[0].name);
    assert_eq!(&result[0].prop_type, &expected_result[0].prop_type);
    assert_eq!(&result[0].entity_name, &expected_result[0].entity_name);
    assert_eq!(&result[0].prefix, &expected_result[0].prefix);
    assert_eq!(&result[1].entity_name, &expected_result[1].entity_name);
    assert_eq!(&result[1].snake_name, &expected_result[1].snake_name);
    assert_eq!(&result[1].camel_name, &expected_result[1].camel_name);
    assert_eq!(&result[1].constant_name, &expected_result[1].constant_name);
    assert_eq!(&result[1].prop_type, &expected_result[1].prop_type);
}

#[test]
fn test_go_prefiix_style_props() {
    let props:String = "pub_name*string,pub_The_age*int".to_string();
    
    let style = PropStyle {
        type_separator: Some(' '),
        prop_place: 1,
        type_place: 2,
        prefix: Some("pub_".to_string())
    };

    let expectation_1 = PropVars {
        name: format!("{}","name"),
        prop_type: format!("{}","string"),
        prefix: format!("{}","pub "),
        entity_name: format!("{}","Name"),
        snake_name: format!("{}","name"),
        camel_name: format!("{}","name"),
        kebab_name: format!("{}","name"),
        constant_name: format!("{}","name".to_uppercase()),
    };

    let expectation_2 = PropVars {
        name: format!("{}","The_age"),
        prop_type: format!("{}","int"),
        prefix: format!("{}","pub "),
        entity_name: format!("{}","TheAge"),
        snake_name: format!("{}","the_age"),
        camel_name: format!("{}","theAge"),
        kebab_name: format!("{}","the-age"),
        constant_name: format!("{}","the_age".to_uppercase()),
    };

    let mut expected_result: Vec<PropVars> = Vec::new();
    expected_result.push(expectation_1);
    expected_result.push(expectation_2);
    
    let result = env_prop_mapper(&props, &style);
    assert_eq!(result.len(),expected_result.len());
    assert_eq!(&result[0].name, &expected_result[0].name);
    assert_eq!(&result[0].prop_type, &expected_result[0].prop_type);
    assert_eq!(&result[0].entity_name, &expected_result[0].entity_name);
    assert_eq!(&result[0].prefix, &expected_result[0].prefix);
    assert_eq!(&result[1].entity_name, &expected_result[1].entity_name);
    assert_eq!(&result[1].snake_name, &expected_result[1].snake_name);
    assert_eq!(&result[1].camel_name, &expected_result[1].camel_name);
    assert_eq!(&result[1].constant_name, &expected_result[1].constant_name);
    assert_eq!(&result[1].prop_type, &expected_result[1].prop_type);
}
