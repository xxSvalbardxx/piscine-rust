pub fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial(num - 1),
    }
}
