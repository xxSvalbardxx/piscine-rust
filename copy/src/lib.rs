pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let d = c as f64;
    (c, d.exp(), d.abs().ln()) 
}
// returns the exponential function of each value as a string 
pub fn str_function(a: String) -> (String, String) {
    let mut b = String::new();
    for d in a.split_whitespace() {
        let e = d.parse::<f64>().unwrap();
        b.push_str(&e.exp().to_string());
        b.push_str(" ");
    }
    b.pop();
    (a, b)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut c = Vec::new();
    for d in &b {
        let e = *d as f64;
        c.push(e.abs().ln());
    }
    (b, c) 
}
