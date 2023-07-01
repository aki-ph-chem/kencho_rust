use proconio::input;

fn main() {
    input! {
        n: usize,
        array: [i32; n],
    }

    let inf = 1<<30_i32;
    let mut min_value = inf;
    let mut index_of_min = 0;

    for (index, value) in array.iter().enumerate() {
        if *value < min_value {
            min_value = *value;
            index_of_min = index;
        }
    } 

    println!("{} {}", index_of_min, min_value);
}
