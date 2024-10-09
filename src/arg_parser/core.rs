use crate::arg_parser::types::{OptionData, OptionResult, ArgRouteError};
use crate::arg_parser::io::index_options;
use serde_json::json;

pub fn process_args(args: &[String], options: &[OptionData]) -> Result<OptionResult, ArgRouteError> {
    if args.len() < 2 {
        return Err(ArgRouteError {
            message: "No arguments provided".to_string(),
        });
    }

    let (short_options, long_options) = index_options(options);

    let key = &args[1];
    let option_data = short_options
        .get(key)
        .or_else(|| long_options.get(key))
        .ok_or_else(|| ArgRouteError {
            message: format!("option {} not exist", key),
        })?;

    let params = if args.len() > 2 {
        let name = &args[2];

        if option_data.has_props {
            if args.len() > 3 {
                let props = &args[3];
                json!({
                    "name": name,
                    "props": props
                })
            } else {
                return Err(ArgRouteError {
                    message: format!("option {} requires properties", key),
                });
            }
        } else {
            json!({
                "name": name
            })
        }
    } else {
        return Err(ArgRouteError {
            message: format!("option {} requires a name", key),
        });
    };

    Ok(OptionResult {
        short_option: option_data.short_option.clone(),
        option: option_data.option.clone(),
        template: option_data.template.clone(),
        params,
    })
}
