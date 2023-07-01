fn gcd(m: i32, n: i32) -> i32 {
    match n {
        0 => m,
        _ => gcd(n, m % n),
    }
}
fn main() {
    println!("gcd(51, 15) = {}", gcd(51,15));
    println!("gcd(15, 51) = {}", gcd(15,51));
    println!("gcd(100, 50) = {}", gcd(100,50));
}
