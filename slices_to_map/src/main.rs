use slices_to_map::*;

fn main() {
	let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
	let values = [1, 3, 23, 5, 2];
	println!("{:?}", slices_to_map(&keys, &values));
}