pub fn abbreviate(phrase: &str) -> String {
    phrase
        .trim()
        .replace(",", " ")
        .replace("-", " ")
        .replace("_", " ")
        .split_whitespace()
        .map(|token| {
            let input = if is_only_uppercase(token) {
                token.to_lowercase()
            } else {
                token.to_owned()
            };
            let mut chars = input.chars();
            let new_word = match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            };
            new_word
        })
        .map(|word| {
            word.chars()
                .filter(|char| char.is_uppercase())
                .collect::<String>()
        })
        .collect()
}

fn is_only_uppercase(input: &str) -> bool {
    let mut result = true;
    for ch in input.chars() {
        if ch.is_lowercase() {
            result = false;
            break;
        }
    }
    result
}
