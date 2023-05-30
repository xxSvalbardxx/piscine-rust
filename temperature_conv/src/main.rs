use temperature_conv::*;

fn main() {
    println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
    println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
}
