pub fn snake_to_pascal_case(input: &str) -> String {
    input
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

pub fn snake_to_camel_case(input: &str) -> String {
    input
        .split('_')
        .enumerate()
        .map(|(i, word)| {
            if i == 0 {
                word.to_lowercase()
            } else {
                word[..1].to_uppercase() + &word[1..].to_lowercase()
            }
        })
        .collect()
}