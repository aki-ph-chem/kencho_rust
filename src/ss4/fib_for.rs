fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut fib = 0;
            let mut fib_0 = 0; 
            let mut fib_1 = 1; 
            for _i in 2 .. n + 1 {
                fib = fib_0 + fib_1;
                fib_0 = fib_1;
                fib_1 = fib;
            }
            fib
        },
    }
}

fn main() { 
    println!("fib(6) = {}", fib(6));
}
