
// #debug
#[derive(Debug)]


pub struct Matrix(pub(i32, i32), pub(i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}


#[cfg(test)]
mod tests {
    use matrix_transposition::transpose;
    use matrix_transposition::Matrix;

    fn main() {
        let matrix = Matrix((1, 3), (4, 5));
        println!("Original matrix {:?}", matrix);
        println!("Transpose matrix {:?}", transpose(matrix));
    }
}
