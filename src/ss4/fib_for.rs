fn fib(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let mut fib = 0;
            let mut fib_1 = 1; 
            let mut fib_2 = 1; 
            for _i in 2 .. n + 1 {
                fib = fib_1 + fib_2;
                fib_1 = fib_2;
                fib_2 = fib;
            }
            fib
        },
    }
}

fn main() { 
    println!("fib(6) = {}", fib(6));
}
