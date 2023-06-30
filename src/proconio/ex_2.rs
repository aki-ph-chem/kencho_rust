use proconio::input;

fn main() {
    input! {
        n: usize,
        mut array:[i32; n], 
    }

    for (index, value) in array.iter().enumerate() {
        println!("(index, value) = ({}, {})", index, value);
    } 

}
