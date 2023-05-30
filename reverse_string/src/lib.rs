pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect::<String>()
    // .chars() returns an iterator over the characters of a string slice
    // .rev() reverses the iterator
    // .collect() collects the iterator into a collection wich in this case is a String
    // .collect::<String>() is a turbofish operator that tells the compiler what type to collect into
    // turbofish operator is ::<>
    // ::<String> is optional because the compiler can infer the type from the context
}


#[cfg(test)]
mod tests {
    use reverse_string::rev_str;

fn main() {
    println!("{}", rev_str("Hello, world!"));
    println!("{}", rev_str("Hello, my name is Roman"));
    println!("{}", rev_str("I have a nice car!"));
    println!("{}", rev_str("How old are You"));
    println!("{}", rev_str("ex: this is an example água"));
}
}
