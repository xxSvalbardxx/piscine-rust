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
