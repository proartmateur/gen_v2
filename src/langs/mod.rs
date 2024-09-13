pub mod en;
pub mod es;

use std::collections::HashMap;
use std::sync::LazyLock;

pub fn get_lang(lang: &String) -> &'static LazyLock<HashMap<&'static str, &'static str>> {
    match lang.as_str() {
        "es" => &es::ES_STRINGS,
        "en" => &en::EN_STRINGS,
        _ => &en::EN_STRINGS,
    }
}
