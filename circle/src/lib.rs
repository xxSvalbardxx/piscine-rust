use std::f64::consts;

#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x: f64,y: f64 , radius: f64,) -> Self {
        Circle {
            center: Point::new(x,y),
            radius: radius,
        }
    }
    pub fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn intersect(&self, other: &Circle) -> bool {
        self.center.distance(&other.center) < self.radius + other.radius
    }

}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64,) -> Self {
        Self {
            x,
            y,
        }
    }
    pub fn distance(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x * x + y * y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;
    use circle::{Circle, Point};
#[test]
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
}
