use proconio::input;

fn main() {
    input!{
        a: [i32;6],
        mut value: i32,
    }
    let coins = vec![500, 100, 50, 10, 5, 1];

    let mut res = 0;
    for i in 0 .. 6 {
        let q = value / coins[i];
        if q < a[i] {
            res += q;
            value -= q * coins[i];
        } else {
            res += a[i];
            value -= a[i] * coins[i];
        }
    }
    println!("{}", res);
}
