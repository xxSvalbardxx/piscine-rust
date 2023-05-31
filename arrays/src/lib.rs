pub fn sum(a: &Vec<i32>) -> i32 {
	//type of argument missing in the signature here
    let mut sum = 0;
    for i in a {
        sum += i;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let mut a = [0; 32];
    for i in 0..32 {
        a[i] = 10;
    }
    a


}
