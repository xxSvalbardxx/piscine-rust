pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..vec.len() {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}