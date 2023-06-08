
extern crate case;


pub fn expected_variable(original: &str, expected: &str) -> Option<String>{
    if original.contains(" ") {
        return None;
    }
    // println!("{}",!original.chars().any(|c| c.is_ascii_uppercase()));
    if !original.contains('_') && !original.chars().any(|c| c.is_ascii_uppercase()) {
        None
    } else {
        
        let diff = edit_distance(&original.to_lowercase(),&expected.to_lowercase());
        if diff > original.len() {
            return None;
        }
        let bigger = std::cmp::max(original.len(), expected.len());
        let res = ((bigger-diff)*100) as f64/bigger as f64;
        let resu = res.ceil();
        if resu < 50.0 {
            return None;
        }
        return Some(resu.to_string()+&"%".to_string());
    }
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let w1 = source.chars().collect::<Vec<_>>();
    let w2 = target.chars().collect::<Vec<_>>();
 
    let source_length = w1.len() + 1;
    let target_length = w2.len() + 1;
 
    let mut matrix = vec![vec![0; source_length]; target_length];
 
    for i in 1..source_length { matrix[0][i] = i; }
    for j in 1..target_length { matrix[j][0] = j; }
 
    for j in 1..target_length {
        for i in 1..source_length {
            let x: usize = if w1[i-1] == w2[j-1] {
                matrix[j-1][i-1]
            } else {
                1 + std::cmp::min(
                        std::cmp::min(matrix[j][i-1], matrix[j-1][i])
                        , matrix[j-1][i-1])
            };
            matrix[j][i] = x;
        }
    }
    matrix[target_length-1][source_length-1]
}