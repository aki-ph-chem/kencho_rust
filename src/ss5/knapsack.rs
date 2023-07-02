use proconio::input;

fn chmax<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn dp_a(v: &Vec<i64>, w: &Vec<i64>, dp: &mut Vec<Vec<i64>>
        , w_max: i64) -> i64 {

    for i in 0 .. v.len() {
        for u in 0 .. w_max + 1 {
            if u < w[i] {
                dp[i + 1][u as usize] = dp[i][u as usize];
            } else {
                dp[i + 1][u as usize] = std::cmp::max(dp[i][u as usize], dp[i][(u - w[i]) as usize] + v[i]);
            }
        }
    }
    dp[v.len()][w_max as usize]
}

fn dp_b(v: &Vec<i64>, w: &Vec<i64>, dp: &mut Vec<Vec<i64>>
        , w_max: i64) -> i64 {

    for i in 0 .. v.len() {
        for u in 0 .. w_max + 1 {
            if u >= w[i] {
                let tmp =  dp[i][(u - w[i]) as usize] + v[i];
                chmax(&mut dp[i + 1][u as usize], tmp);
            } 
            let tmp = dp[i][u as usize];
            chmax(&mut dp[i + 1][u as usize], tmp);
        }
    }
    dp[v.len()][w_max as usize]
}

fn main() {
    input!{
        n: usize,
        v: [i64; n],
        w: [i64; n],
        w_max: i64,
    }

    let mut dp_1: Vec<Vec<i64>> = vec![vec![0; w_max as usize + 1]; n + 1];
    println!("{}", dp_a(&v, &w, &mut dp_1, w_max));

    let mut dp_2: Vec<Vec<i64>> = vec![vec![0; w_max as usize + 1]; n + 1];
    println!("{}", dp_b(&v, &w, &mut dp_2, w_max));
}
