fn condition(index: usize, key: i32, array: &Vec<i32>) -> bool {
    if array[index] >= key {
        true
    } else {
        false
    }
}

fn bin_search(key: i32, array: &Vec<i32>) -> usize {
    let mut left = 0; let mut right = array.len();

    while right - left > 1 {
        let mid = (left + right) / 2;
        if condition(mid, key, &array) {
            right = mid;
        } else {
            left = mid;
        }
    }  
    right
}

fn main() {
    let a = vec![1,3,4,20, 22,23,30,40];

    println!("condition({},{}, &a) = {}",
    3, 30, condition(3, 30, &a));
    println!("condition({},{}, &a) = {}",
    1, 30, condition(1, 30, &a));
    println!("condition({},{}, &a) = {}",
    4, 4, condition(4, 4, &a));

    println!("bin_search({}, &a) = {}",
    5, bin_search(5, &a));
    println!("bin_search({}, &a) = {}",
    20, bin_search(20, &a));
    println!("bin_search({}, &a) = {}",
    30, bin_search(30, &a));
}
