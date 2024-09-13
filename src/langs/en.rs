use std::collections::HashMap;
use std::sync::LazyLock;

pub static EN_STRINGS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("app_info.description", r#"Make your project faster with your own architecture!"#),
        ("app_info.credits", "Developed by author with ❤️ from México. year"),
    ])
});