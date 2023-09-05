use proconio::input;

fn main() {
    input!{
        n: usize,
        mut array: [(i64, i64);n],
    }

    let mut sum = 0;
    for i in (0..n).rev() {
        array[i].0 += sum; // 前回までの操作回数を足す
        let r = array[i].0 % array[i].1;
        let mut d = 0;
        if r != 0 {
            d = array[i].1 - r;
        }
        sum += d;
    }
    println!("{}", sum);
}
