use proconio::input;

fn chmin<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

// 普通の再帰
fn frog(h: &Vec<i64>, i: usize) -> i64 {
    let mut res = 1<<30_i64;
    match i {
        0 => 0,
        _ => {
            let tmp = frog(h, i -1) + (h[i] - h[i - 1]).abs(); 
            chmin(&mut res, tmp);
            if i > 1 {
                let tmp = frog(h, i - 2) + (h[i] - h[i - 2]).abs(); 
                chmin(&mut res, tmp);
            }
            res
        }
    }
}

// メモ化再帰
fn frog_memo(h: &Vec<i64>, dp: &mut Vec<i64> , i: usize) -> i64 {
    let inf = 1<<30_i64;
    match i {
        0 => 0,
        _ => {
            if dp[i] < inf {
                dp[i]
            } else {
                let tmp = frog_memo(h, dp, i - 1) + (h[i] - h[i - 1]).abs();
                chmin(&mut dp[i], tmp);
                if i > 1 {
                    let tmp = frog_memo(h, dp, i - 2) + (h[i] - h[i - 2]).abs();
                    chmin(&mut dp[i], tmp);
                }
                dp[i]
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    println!("frog(&h, h.len() - 1) = {}", frog(&h, h.len() - 1));

    let inf = 1<<30_i64;
    let mut dp:Vec<i64> = vec![inf;n];
    println!("frog_memo(&h, &mut dp, h.len() - 1) = {}", frog_memo(&h, &mut dp, h.len() - 1));
}
