use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut frequency_count = HashMap::new();
    for word in words {
        let count = frequency_count.entry(word).or_insert(0); // if word is not in the map, insert 0. Otherwise, return the value
        *count += 1;
    }
    frequency_count
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}