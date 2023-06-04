use collect::*;

fn main() {
	let ref mut v = vec![3, 2, 4, 5, 1, 7];
	let mut b = v.clone();
	bubble_sort(v);
	println!("{:?}", v);

	b.sort();
	println!("{:?}", b);
}