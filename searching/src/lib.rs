pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().position(|&x| x == key)
}