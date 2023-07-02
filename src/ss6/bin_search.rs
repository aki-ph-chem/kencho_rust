use proconio::input;

fn bin_search(array: &Vec<i32>, key: i32) -> Option<usize> {
    let mut left = 0; let mut right = array.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if key == array[mid] {
            return Some(mid);
        } 

        if key < array[mid] {
            right = mid - 1;
        } 

        if key > array[mid] {
            left = mid + 1;
        }
    }
    None
}

fn main() {
    input!{
        n: usize,
        array: [i32;n],
        key: i32,
    } 

    match bin_search(&array, key) {
        Some(index) => println!("{}",index),
        None => println!("{}", -1),
    }
}
