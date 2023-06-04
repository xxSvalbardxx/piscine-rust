pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_vec: Vec<char> = s1.chars().collect();
    let mut s2_vec: Vec<char> = s2.chars().collect();
    s1_vec.sort();
    s2_vec.sort();
    s1_vec == s2_vec
    
}
