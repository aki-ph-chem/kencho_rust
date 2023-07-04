use proconio::input;

fn chmin<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    if *a > b {
       *a = b;
    }
}

fn main() {
    input! {
        s: String,
        t: String
    }

    let inf = 1<<30_i32;
    // dp table
    let mut dp = vec![vec![inf; t.len() + 1]; s.len() + 1];
    dp[0][0] = 0;

    for i in 0 .. s.len() + 1 {
        for j in 0 .. t.len() + 1 {
            
            // 変更
            if i > 0 && j > 0 {
                if s.chars().nth(i - 1).unwrap() == t.chars().nth(j - 1).unwrap() {
                    let tmp =  dp[i - 1][j - 1];
                    chmin(&mut dp[i][j], tmp);
                } else {
                    let tmp =  dp[i - 1][j - 1] + 1;
                    chmin(&mut dp[i][j], tmp);
                }
            }         

            // 削除
            if i > 0 {
                let tmp = dp[i - 1][j] + 1;
                chmin(&mut dp[i][j], tmp);
            }

            // 挿入
            if j > 0 {
                let tmp = dp[i][j - 1] + 1;
                chmin(&mut dp[i][j], tmp);
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
