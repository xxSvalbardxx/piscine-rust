#[derive(Debug)]
pub struct Person<'a> {
	pub name: &'a str,
	pub age: u8,
}

impl Person <'_>{
	pub fn new(name: &str) -> Person{
		Person {
			name: name,
			age: 0,
		}
	}
}