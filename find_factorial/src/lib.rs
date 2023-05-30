pub fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial(num - 1),
    }
}

#[cfg(test)]
mod tests {
    use find_factorial::factorial;

    fn main() {
        println!("The factorial of 0 = {}", factorial(0));
        println!("The factorial of 1 = {}", factorial(1));
        println!("The factorial of 5 = {}", factorial(5));
        println!("The factorial of 10 = {}", factorial(10));
        println!("The factorial of 19 = {}", factorial(19));
    }
}
