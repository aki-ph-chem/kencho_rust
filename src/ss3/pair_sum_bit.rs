use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i32,
        array: [i32; n],
    }

    let mut is_exist = false; 

    // bit: 0 ~ 2^n - 1
    for bit in 0..1<<n {
        let mut sum = 0;
        for i in 0..n {
            if (bit & (1 << i)) != 0 {
                sum += array[i];
            }
            if sum == w {
                is_exist = true;
            }
        }
    } 

    if is_exist {
        println!("Yes");
    } else {
        println!("No");
    }
}
