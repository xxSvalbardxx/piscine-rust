// insert a new element at the end of the vector
pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val); // push() is a method of Vec<T>
}

// return the element at the given index
pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    vec[index].clone() // clone() is needed because String does not implement Copy trait
}
