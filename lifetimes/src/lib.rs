#[derive(Debug)]
pub struct Person<'a> { // <'a> is a lifetime annotation that says the struct cannot outlive the reference it holds
	pub name: &'a mut str, // &str is a reference to a string slice (a string literal) that cannot be mutated (immutable)
	pub age: u8,
}

impl Person<'_> {
	pub fn new<'a>(name: &'a mut str) -> Person<'a>{
		Person {
			name: name,
			age: 0,
		}
	}
}