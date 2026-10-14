/// Lowercase, fold ё/е, and collapse Unicode whitespace without a token Vec.
pub fn normalize_search_text(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut separator = false;
    for character in value.chars() {
        if character.is_whitespace() {
            separator = !result.is_empty();
            continue;
        }
        if separator {
            result.push(' ');
            separator = false;
        }
        for lower in character.to_lowercase() {
            result.push(if lower == 'ё' { 'е' } else { lower });
        }
    }
    result
}
