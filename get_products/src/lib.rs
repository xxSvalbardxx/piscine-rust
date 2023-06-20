pub fn get_products(mut arr: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    if arr.len() <= 1 {
        return res;
    }
        for i in &arr {
            let mut temp = 1;
            for j in &arr {
                if i != j {
                    temp *= j;
                }
            }
            res.push(temp);
        }
    res
}