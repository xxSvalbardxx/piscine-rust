use circle::{Circle, Point};

fn main() {
	let circle = Circle::new(500.0, 500.0, 150.0);
	let circle1 = Circle {
		center: Point { x: 80.0, y: 115.0 },
		radius: 30.0,
	};
	let point_a = Point { x: 1.0, y: 1.0 };
	let point_b = Point { x: 0.0, y: 0.0 };
	println!("circle = {:?} area = {}", circle, circle.area());
	println!("circle = {:?} diameter = {}", circle, circle.diameter());
	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
	println!(
		"circle and circle1 intersect = {}",
		circle.intersect(&circle1)
	);

	println!(
		"distance between {:?} and {:?} is {}",
		point_a,
		point_b,
		point_a.distance(&point_b)
	);

}