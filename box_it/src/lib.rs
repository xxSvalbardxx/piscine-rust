pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut v: Vec<u32> = Vec::new();
    let split = s.split_whitespace();
    for i in split {
        // &s[prefix.len()..])
        if i.ends_with("k") {
            let mut temp = i.to_string();
            temp.pop();
            let temp = temp.parse::<f32>().unwrap();
            let temp = (temp * 1000.0) as u32;
            v.push(temp);
        } else {
            let temp = i.parse::<u32>().unwrap();
            v.push(temp);
        }
    }
    Box::new(v)

}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
