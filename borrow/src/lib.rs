pub fn str_len(s: &str) -> usize { // watch diff btw String and &str
    s.len()
}

#[cfg(test)]
mod tests {
    use borrow::*;

fn main() {
	let s = 56;
	let s1 = "camelCase".to_string();

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
}
}
