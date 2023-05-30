use borrow_me_the_reference::*;

fn main() {
	let mut a = String::from("+e+a+");
	let mut b: Vec<String> = vec![String::from("2+2"), String::from("3+2"), String::from("10-3"), String::from("5+5")];
	delete_and_backspace(&mut a);
	do_operations(&mut b);
	println!("{:?}", (a,b));
}
