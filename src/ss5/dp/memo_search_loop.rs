// dp[MAX_N -1][0]からスタートするdp
fn dp_loop(v: &Vec<i32>, w: &Vec<i32>, dp: &mut Vec<Vec<i32>>, w_max: i32) -> i32 {
    for i in (0 .. v.len()).rev() {
        for u in 0 .. w_max + 1 {
            if u < w[i] {
                dp[i][u as usize] = dp[i + 1][u as usize];
            } else {
                dp[i][u as usize] = std::cmp::max(dp[i + 1][u as usize], dp[i + 1][(u - w[i]) as usize ] + v[i]);
            }
        }
    }
    dp[0][w_max as usize]
}

// dp[0][0]からスタートするdp
fn dp_loop_2(v: &Vec<i32>, w: &Vec<i32>, dp: &mut Vec<Vec<i32>>, w_max: i32) -> i32 {
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

// dp[MAX_N - 1][w_max]からスタートするdp
fn dp_loop_3(v: &Vec<i32>, w: &Vec<i32>, dp: &mut Vec<Vec<i32>>, w_max: i32) -> i32 {
    for i in (0 .. v.len()).rev() {
        for u in (0 .. w_max + 1).rev() {
            if u < w[i] {
                dp[i][u as usize] = dp[i][u as usize];
            } else {
                dp[i][u as usize] = std::cmp::max(dp[i + 1][u as usize], dp[i + 1][(u - w[i]) as usize] + v[i]);
            }
        }
    }

    dp[0][w_max as usize]
}

fn main() {
    const MAX_N: usize = 100;
    let v = vec![2, 1, 3, 2];
    let w = vec![3, 2, 4, 2];

    // dp[MAX_N -1][0]からスタート
    let mut dp: Vec<Vec<i32>> = vec![vec![0;MAX_N]; MAX_N];
    println!("dp_loop(&v, &w, &mut dp, 10) = {}",
    dp_loop(&v, &w, &mut dp, 10));

    // dp[0][0]からスタート
    let mut dp_2: Vec<Vec<i32>> = vec![vec![0;MAX_N]; MAX_N];
    println!("dp_loop(&v, &w, &mut dp_2, 10) = {}",
    dp_loop_2(&v, &w, &mut dp_2, 10));

    // dp[MAX_N - 1][MAX_N - 1]からスタート
    let mut dp_3: Vec<Vec<i32>> = vec![vec![0;MAX_N]; MAX_N];
    println!("dp_loop(&v, &w, &mut dp_2, 10) = {}",
    dp_loop_3(&v, &w, &mut dp_3, 10));
}
