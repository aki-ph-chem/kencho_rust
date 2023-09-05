use proconio::input;

fn main() {
    input!{
        mut value: i32,
        a: [i32;6],
    }
    let coins = vec![500, 100, 50, 10, 5, 1];

    let mut res = 0;
    for i in 0 .. 6 {
        let mut q = value / coins[i];
        if q > a[i] {
            q = a[i];
        }

        value -= coins[i] * q; 
        res += q;
    }
    println!("{}", res);
}
