#[derive(Debug)]
pub struct Person<'a> {
	pub name: &'a str,
	pub age: u8,
}

impl Person<'_> {
	pub fn new<'a>(name: &'a str) -> Person<'a>{
		Person {
			name: name,
			age: 0,
		}
	}
}