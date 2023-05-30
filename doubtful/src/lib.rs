pub fn doubtful(s: &mut String) {
    *s+="?";
}

#[cfg(test)]
mod tests {
    use doubtful::*;

    #[test]
    fn main() {
        let mut s = String::from("Hello");
    
        println!("Before changing the string: {}", s);
    
        doubtful(/*add your code here*/);
    
        println!("After changing the string: {}", s);
    }
}
