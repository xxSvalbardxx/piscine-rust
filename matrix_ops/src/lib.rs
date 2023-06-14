pub use lalgebra_scalar::Scalar;

use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone + std::ops::Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None; // Matrices have different dimensions, cannot be added
        }

        let mut result = Vec::new();

        for i in 0..self.0.len() {
            let mut row = Vec::new();

            for j in 0..self.0[i].len() {
                let sum = self.0[i][j].clone() + other.0[i][j].clone();
                row.push(sum);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T: Clone + std::ops::Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut rslt = Vec::new();
        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[i].len() {
                let diff = self.0[i][j].clone() - other.0[i][j].clone();
                row.push(diff);
            }
            rslt.push(row);
        }
        Some(Matrix(rslt))
    }
}
