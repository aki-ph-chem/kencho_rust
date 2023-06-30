use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        array: [[i32;n]; m],
    }

    for i in 0..n {
        for j in 0..m {
            println!("array[{}][{}] = {}", i,j, array[i][j]);
        }
    }
}
