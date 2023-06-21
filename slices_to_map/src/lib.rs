pub fn slices_to_map(&keys [T], &values [U]) -> HashMap<&T, &U> {
    let mut map = HashMap::new();
    
    if keys.len() != values.len() {
        // take the shorter of the two slices
        let len = if keys.len() < values.len() { keys.len() } else { values.len() };
        for i in 0..len {
            map.insert(keys[i], values[i]);
        }
    } else {
        for i in 0..keys.len() {
            map.insert(keys[i], values[i]);
        }
    }
    map
}