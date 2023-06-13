use lalgebra_scalar::Scalar;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);




impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
		Matrix(vec![vec![T::zero(); 1]; 1])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        // filled by only zeros
		Matrix(vec![vec![T::zero(); row]; col])
        
	}

	pub fn identity(n: usize) -> Matrix<T> {
		// declare an identity matrix. wich is a square matrix with 1s on the main diagonal and 0s elsewhere
		let mut m = Matrix::zero(n, n);
		for i in 0..=n {
			m.0[i][i] = T::one(); // m.0 means that we are accessing the first element of the tuple.
		}
		m
	}
}