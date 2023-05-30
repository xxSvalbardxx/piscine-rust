pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}


#[cfg(test)]
mod tests {
    use division_and_remainder::divide;

fn main() {
    let x = 9;
    let y = 4;
    let (division, remainder) = divide(x, y);
    println!(
        "{}/{}: division = {}, remainder = {}",
        x, y, division, remainder
    );
}
}
