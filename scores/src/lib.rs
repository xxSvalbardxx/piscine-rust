
pub fn score(word: &str) -> u64 {
    let mut total_score: u64 = 0;
    for c in word.chars() {
        let lowercase_char = c.to_lowercase().next().unwrap();
        let score = match lowercase_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0, // Handle any other characters that don't have a value
        };
        total_score += score;
    }
    total_score
}