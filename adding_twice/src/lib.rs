
pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub fn twice<F>(f: F) -> impl Fn(i32) -> i32 
where
    F: Fn(i32) -> i32,
{
    move |x| f(x) + f(0)
}

