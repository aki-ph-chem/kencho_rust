use proconio::input;

fn chmin<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    if *a > b{
        *a = b;
    }
}

// 貰う形式(pull-based)のdp
fn dp_pull_based(h: & Vec<i64>, dp: &mut Vec<i64>) -> i64 {
    for i in 1 .. h.len() {
        let tmp = dp[i - 1] + (h[i] - h[i - 1]).abs();
        chmin(&mut dp[i], tmp);
        if i > 1{
            let tmp =  dp[i - 2] + (h[i] - h[i - 2]).abs();
            chmin(&mut dp[i], tmp);
        }
    }
    dp[h.len() - 1]
}

// 配る形式(push-based)のdp
fn dp_push_based(h: & Vec<i64>, dp: &mut Vec<i64>) -> i64 {
    let n = h.len();
    for i in 0 .. n {
        if  i + 1 < n  {
            let tmp = dp[i] + (h[i] - h[i + 1]).abs();
            chmin(&mut dp[i + 1], tmp);
        }
        if i + 2 < n {
            let tmp = dp[i] + (h[i] - h[i + 2]).abs();
            chmin(&mut dp[i + 2], tmp);
        }
    }

    dp[n - 1]
}

fn main() {
    input! {
        n: usize,
        h: [i64;n],
    }

    let inf = 1<<30_i64;

    // 貰う形式
    let mut dp_1 = vec![inf; n];
    dp_1[0] = 0;
    println!("{}",dp_pull_based(&h, &mut dp_1));

    // 配る形式
    let mut dp_2 = vec![inf; n];
    dp_2[0] = 0;
    println!("{}",dp_push_based(&h, &mut dp_2));
}
