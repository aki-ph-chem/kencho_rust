fn sum(n: i32) -> i32 {
    match n {
        0 => 0,
        _ => n + sum(n - 1), 
    }
}
fn main() {
    println!("sum(3) = {}", sum(3));
    println!("sum(10) = {}", sum(10));
}
