use proconio::input;

fn condition(index: usize, key: i32, array: &Vec<i32>) -> bool {
    if array[index] >= key {
        true
    } else {
        false
    }
}

fn bin_search(key: i32, array: &Vec<i32>) -> (usize, i32) {
    let mut left = 0; let mut right = array.len();

    while right - left > 1 {
        let mid = (left + right) / 2;
        if condition(mid, key, &array) {
            right = mid;
        } else {
            left = mid;
        }
    }  
    (right, array[right])
}

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        mut b: [i32; n],
    }

    let inf = 1<<30_i32;
    let mut min = inf;
    b.sort();

    for i in 0 .. n {
        let (_index, b_index) = bin_search(k - a[i], &b);
        if (a[i] + b_index ) < min {
            min = a[i] + b_index;
        }
    }

    println!("{}", min);
}
