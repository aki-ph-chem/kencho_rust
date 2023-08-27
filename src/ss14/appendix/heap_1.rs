use std::collections::BinaryHeap;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Pair(i64,usize);

fn main() {
    let array = [1,2,3,5,4];
    let mut q_0 = BinaryHeap::<Pair>::new();
    for (index, value) in array.iter().enumerate() {
        q_0.push(Pair(*value, index))
    }

    while !q_0.is_empty() {
        if let Some(v) = q_0.peek() {
            println!("q: first = {}, q: second = {}", v.0, v.1);
        }
        q_0.pop();
    }
}
