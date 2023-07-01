use proconio::input;

fn pair_sum(n: i32, w: i32, array: &Vec<i32>) -> bool {
    match n {
        0 => {
            if w == 0 {
                true
            } else {
                false
            }
        }
        _ =>  {
            if pair_sum(n - 1, w, array) {
                true
            } else if pair_sum(n - 1, w - array[n as usize - 1], array) {
                true
            } else {
                false
            }
        }
    }
}

fn main() { 
    input! {
        n: usize,
        w: i32,
        array: [i32;n]
    }

    if pair_sum(n as i32, w, &array) {
        println!("Yes");
    } else {
        println!("No");
    }
 }
