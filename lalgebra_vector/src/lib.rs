
pub use lalgebra_scalar::Scalar;
use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.0.len() {
            result.push(self.0[i].clone() + other.0[i].clone());
        }
        Some(Vector(result))
    }


}

impl<T: Scalar + std::ops::Mul<Output = T>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum: T = T::clone(&self.0[0]) * T::clone(&other.0[0]);
        for (a, b) in self.0.iter().skip(1).zip(other.0.iter().skip(1)) { // .zip is a method of Iterator that "zips" two iterators together into a single iterator of pairs
            sum = sum + (T::clone(a) * T::clone(b));
        }
        Some(sum)
    }
}