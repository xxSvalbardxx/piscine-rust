pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
let mut j = 0;
    for i in v.chars(){
        
        if i == pat {

            break;
        }
        j+=1;
    }
    j
}

#[cfg(test)]
mod tests {
    use string_literals::*;

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}
}
