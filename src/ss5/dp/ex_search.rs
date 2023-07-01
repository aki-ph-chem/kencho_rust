fn ex_search(value: &Vec<i32>, weight: &Vec<i32>
             , i: usize, max_weight: i32) -> i32 {
    if i == value.len() {
        0
    } else if max_weight < weight[i] {
        ex_search(value, weight, i + 1, max_weight)
    } else {
        std::cmp::max(
            ex_search(value, weight, i + 1, max_weight),
            ex_search(value, weight, i + 1, max_weight - weight[i])+ value[i])
    }
}

fn main() {
    let value = vec![2, 1, 3, 2];
    let weight = vec![3, 2, 4, 2];

    println!("ex_search(&value, &weight, 0, 10) = {}",
    ex_search(&value, &weight, 0, 10))
}
