use proconio::input;

fn main() {
    input!{
        n: usize,
        h: [i32; n],
        s: [i32; n],
    }

    let inf = 1<<30_i64;
    // 二分探索
    let mut left = 0; let mut right = inf;
    while right - left > 1 {
        let mid = (left + right) / 2;
        // 判定
        let mut ok = true;
        let mut time_limit = vec![0; n]; // 風船を割るまでの時間制限
                                         //
        for i in 0 .. n {
            // midが初期高度より低かったらfalse
            if mid < h[i] {
                ok = false;
            } else {
                time_limit[i] = (mid - h[i]) / s[i];
            } 
        }

        // 時間制限が差し迫っている順にソート
        time_limit.sort();
        for i in 0 .. n {
            if time_limit[i] < i as i32 {
                ok = false; // 時間切れ
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
