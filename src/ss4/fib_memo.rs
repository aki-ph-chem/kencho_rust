fn fib_rec(index: usize,  array: &mut Vec<i64>) -> i64 {
    match index {
        0 => 0,
        1 => 1,
        _ => {
            if array[index] != -1 {
                array[index]
            } else {
                array[index] = fib_rec(index - 1, array) + fib_rec(index - 2, array);
                array[index]
            }
        }
    } 
}

fn fib_dp(n: usize, array: &mut Vec<i64>){
    array[0] = 0; array[1] = 1;
    for i in 2..(n + 1) {
        array[i] = array[i - 1] + array[i - 2];
    } 
}

fn main() {
    const ARRAY_SIZE:usize = 50;

    let mut array:Vec<i64> = vec![-1;ARRAY_SIZE];
    println!("fib_rec(49, &mut array) = {}", fib_rec(49, &mut array));
    for (index, value) in array.iter().skip(2).enumerate()  {
                println!("fib_{} = {}", index + 2, value);
    }
    
    let mut array_2:Vec<i64> = vec![0; ARRAY_SIZE];
    fib_dp(49, &mut array_2);
    for (index, value) in array_2.iter().enumerate()  {
                println!("fib_{} = {}", index, value);
    }
}
