use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut seen_chars = HashSet::new();
    for letter in candidate.chars().filter(|c| c.is_alphanumeric()) {
        for c in letter.to_lowercase() {
            if seen_chars.contains(&c) {
                return false
            }
                seen_chars.insert(c);
        }
    }
    true
}
