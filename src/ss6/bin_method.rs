fn sq_func(x: f64) -> f64 {
    x.powi(2) - 2.0
}

fn sq_func_2(x: f64) -> f64 {
    x.powi(2) - 3.0
} 

fn bin_method(left: f64, right: f64, func: fn(f64) -> f64, iter: i32, ) -> f64 {
    let mut  left = left; let mut right = right;

    for _i in 0 .. iter {
        let mid = (left + right) / 2.0;
        if func(mid) < 0.0 {
            left = mid;
        } else if sq_func(mid) > 0.0 {
            right = mid;
        }
    }

    right
}

fn main() {
    // 二分法でsqrt(2)の値を計算する
    let iter = 1000;
    let mut  left = 0.0; let mut right = 2.0;

    for _i in 0 .. iter {
        let mid = (left + right) / 2.0;
        if sq_func(mid) < 0.0 {
            left = mid;
        } else if sq_func(mid) > 0.0 {
            right = mid;
        }
    }
    println!("(left, right) = ({}, {})", left, right);

    // 二分法でsqrt(3)の値を計算する
    println!("right = {}", bin_method(0.0, 3.0, sq_func_2, 1000));

}
