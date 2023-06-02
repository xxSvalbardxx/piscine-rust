use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let result: f64;
    for i in list {
        sum += i;
    }
    result = sum as f64 / list.len() as f64;
    result
}

pub fn median(list: &Vec<i32>) -> i32 {
    let result: i32;
    let mut sorted_list = list.clone();
    sorted_list.sort();
    if sorted_list.len() % 2 == 0 {
        result = (sorted_list[sorted_list.len() / 2] + sorted_list[sorted_list.len() / 2 - 1]) / 2;
    } else {
        result = sorted_list[sorted_list.len() / 2];
    }
    result
}


// calculate the value that appears most often in a list
pub fn mode(list: &Vec<i32>) -> i32 {
    
    let mut map = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0); // map.entry(i) returns a reference to the value associated with the key i in the map. If there is no value associated with the key, the method inserts the key with a value of 0 and returns a mutable reference to the value.
        *count += 1;
    }
    let mut max_count = 0;
    let mut result = 0;
    for (key, value) in map {
        if value > max_count {
            max_count = value;
            result = *key;
        }
    }
    result
}