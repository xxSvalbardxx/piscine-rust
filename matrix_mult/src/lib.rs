pub use lalgebra_scalar::Scalar;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].to_vec()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut col = Vec::new();

        for i in 0..self.0.len() {
            col.push(self.0[i][n].clone());
        }
        col
    }
}

impl<T: Clone + std::ops::Mul<Output = T> + std::ops::Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }
        // without using the sum crate
        
        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut row = Vec::new();
            for j in 0..other.number_of_cols() {
                let mut sum = self.0[i][0].clone() * other.0[0][j].clone();
                for k in 1..self.number_of_cols() {
                    sum = sum + self.0[i][k].clone() * other.0[k][j].clone();
                }
                row.push(sum);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
