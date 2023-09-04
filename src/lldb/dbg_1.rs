fn sum(n: i32) {
    println!("Inside sum()");
    let mut sum = 0;
    for i in 0..(n+1) {
        sum += i;
    }
    println!("sum = {}", sum);
}

fn fact(n :i32) {
    println!("Inside fact()");
    if n == 0{
        println!("fact = 1");
        return;
    } 
    let mut fact = 1;
    for i in 1..(n+1) {
        fact *= i;
    }
    println!("fact = {}",fact);
}

fn main() {
    sum(3);
    fact(3);
}
