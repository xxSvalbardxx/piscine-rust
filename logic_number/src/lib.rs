pub fn number_logic(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let num_digits = digits.len() as u32;

    let sum_of_powers: u32 = digits.iter().map(|&d| d.pow(num_digits)).sum();

    num == sum_of_powers
}
