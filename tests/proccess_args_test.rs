use gen::arg_parser::types::OptionData;
use gen::arg_parser::core::process_args;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn setup_test_options() -> Vec<OptionData> {
        vec![
            OptionData {
                short_option: "-m".to_string(),
                option: "--model".to_string(),
                template: "model_template.txt".to_string(),
                has_props: false,
            },
            OptionData {
                short_option: "-g".to_string(),
                option: "--group".to_string(),
                template: "group_template.txt".to_string(),
                has_props: true,
            },
        ]
    }

    #[test]
    fn test_valid_model_option() {
        let args = vec!["program".to_string(), "-m".to_string(), "shoe".to_string()];
        let options = setup_test_options();
        let result = process_args(&args, &options).unwrap();
        assert_eq!(result.short_option, "-m");
        assert_eq!(result.option, "--model");
        assert_eq!(result.template, "model_template.txt");
        assert_eq!(result.params, json!({"name": "shoe"}));
    }

    #[test]
    fn test_valid_group_option() {
        let args = vec![
            "program".to_string(),
            "--group".to_string(),
            "test_group".to_string(),
            "a:int,b:float".to_string(),
        ];
        let options = setup_test_options();
        let result = process_args(&args, &options).unwrap();
        assert_eq!(result.short_option, "-g");
        assert_eq!(result.option, "--group");
        assert_eq!(result.template, "group_template.txt");
        assert_eq!(
            result.params,
            json!({"name": "test_group", "props": "a:int,b:float"})
        );
    }

    #[test]
    fn test_invalid_option() {
        let args = vec![
            "program".to_string(),
            "-x".to_string(),
            "invalid".to_string(),
        ];
        let options = setup_test_options();
        let result = process_args(&args, &options);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "option -x not exist");
    }

    #[test]
    fn test_missing_name_for_model() {
        let args = vec!["program".to_string(), "--model".to_string()];
        let options = setup_test_options();
        let result = process_args(&args, &options);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "option --model requires a name"
        );
    }

    #[test]
    fn test_missing_props_for_group() {
        let args = vec![
            "program".to_string(),
            "-g".to_string(),
            "test_group".to_string(),
        ];
        let options = setup_test_options();
        let result = process_args(&args, &options);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "option -g requires properties");
    }

    #[test]
    fn test_no_arguments() {
        let args = vec!["program".to_string()];
        let options = setup_test_options();
        let result = process_args(&args, &options);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "No arguments provided");
    }

    #[test]
    fn test_long_option_name() {
        let args = vec![
            "program".to_string(),
            "--model".to_string(),
            "car".to_string(),
        ];
        let options = setup_test_options();
        let result = process_args(&args, &options).unwrap();
        assert_eq!(result.short_option, "-m");
        assert_eq!(result.option, "--model");
        assert_eq!(result.params, json!({"name": "car"}));
    }
}
