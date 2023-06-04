use std::collections::HashMap;


pub fn bigger(h : HashMap<&str, i32>) -> i32 {
    let mut biggest = 0;
    for (_, v) in h {
        if v > biggest {
            biggest = v;
        }
    }
    biggest
}