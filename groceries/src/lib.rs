// insert a new element at the end of the vector
pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val); // push() is a method of Vec<T>
}

// return the element at the given index
pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    vec[index].clone() // clone() is needed because String does not implement Copy trait
}

#[cfg(test)]
mod tests {
    use groceries::{at_index, insert};

    fn main() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        insert(&mut groceries, String::from("nuts"));
        println!("The groceries list contains {:?}", &groceries);
        println!(
            "The second element of the grocery  list is {:?}",
            at_index(&groceries, 1)
        );
    }
}
