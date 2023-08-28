use proconio::input;

const INF: i64 = 1 << 60;

fn main() {
    //n: 頂点数
    //m: 辺数
    //s: 始点
    input! {
        n: usize,
        m: usize,
        pair: [(usize,usize, i64); m],
    }

    // dp table
    let mut dp = vec![vec![INF; n]; n];
    // 初期化
    for (a, b, w) in pair {
        dp[a][b] = w;
    }
    for v in 0..n {
        dp[v][v] = 0;
    }

    // dpの遷移
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = std::cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    // 結果出力
    let mut exist_negative_cycle = false;
    for v in 0..n {
        if dp[v][v] < 0 {
            exist_negative_cycle = true;
        }
    }

    if exist_negative_cycle {
        println!("NEGATIVE CYCLE");
    }
    // 負閉路がない場合に結果を出力
    else {
        for i in 0..n {
            for j in 0..n {
                if j != 0 {
                    print!(" ");
                }
                if dp[i][j] < INF / 2 {
                    print!("{}", dp[i][j])
                } else {
                    print!("INF");
                }
            }
            println!("");
        }
    }
}
