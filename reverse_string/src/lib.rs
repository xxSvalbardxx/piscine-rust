pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect::<String>()
    // .chars() returns an iterator over the characters of a string slice
    // .rev() reverses the iterator
    // .collect() collects the iterator into a collection wich in this case is a String
    // .collect::<String>() is a turbofish operator that tells the compiler what type to collect into
    // turbofish operator is ::<>
    // ::<String> is optional because the compiler can infer the type from the context
}


