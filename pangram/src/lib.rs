pub fn is_pangram(s: &str) -> bool {
    let s = s.to_lowercase();
    ('a'..='z').all(|c| s.contains(c))
}