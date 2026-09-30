pub fn normalize_search_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .map(|character| if character == 'ё' { 'е' } else { character })
        .collect()
}
