use std::collections::HashMap;

pub fn slices_to_map<'a, T : std::hash::Hash + std::cmp::Eq, U>(keys: &'a [T],values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    for (key, value) in keys.iter().zip(values.iter()) {
        map.insert(key, value);
    }
    map
}