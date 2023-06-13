
pub trait Scalar:
std::ops::Add<Output = Self>
+ std::ops::Sub<Output = Self>
+ std::ops::Mul<Output = Self>
+ std::ops::Div<Output = Self>
+ std::clone::Clone // Clone is a trait that implements the clone() method to indicate that if you clone the current type, you should get the same type back.
+ Sized
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}