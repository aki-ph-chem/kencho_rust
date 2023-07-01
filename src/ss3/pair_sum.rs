use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        array_1: [i32; n],
        array_2: [i32; n],
    }

    let inf = 1<<30_i32;
    let mut min_value = inf;

    for a_1 in &array_1 {
        for a_2 in &array_2 {
            let sum = a_1 + a_2;
            if sum < k {
                continue;
            }

            if sum < min_value {
                min_value = sum;
            }
        }
    }

    println!("{}", min_value);
}
