
// must use (9/5) instead of (9.0/5.0) because the latter is not a constant expression
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32 as f64) / (9 as f64 / 5 as f64)
}
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    9 as f64 / 5 as f64 * c + 32 as f64
}


#[cfg(test)]
mod tests {
    use temperature_conv::*;
    #[test]
    fn main() {
        println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
        println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
    }
}
