fn search(v: &Vec<i32>, w: &Vec<i32>, memo: &mut Vec<Vec<i32>>, 
          i: usize, max_weight: i32) -> i32 {
    if memo[i][max_weight as usize] != 0 {
       memo[i][max_weight as usize] 
    } else if i == v.len() {
        0
    } else if max_weight < w[i] {
        memo[i][max_weight as usize] = search(v, w, memo, i + 1, max_weight);
        memo[i][max_weight as usize]
    } else {
        memo[i][max_weight as usize] = std::cmp::max(
            search(v, w, memo, i + 1, max_weight),
            search(v, w, memo, i + 1, max_weight - w[i]) + v[i]
                     );
        memo[i][max_weight as usize]
    }
}

fn main() {
    const N: usize = 11;
    let mut memo: Vec<Vec<i32>> = vec![vec![0;N];N];

    let value = vec![2, 1, 3, 2];
    let weight = vec![3, 2, 4, 2];

    println!("ex_search(&value, &weight, 0, 10) = {}",
    search(&value, &weight,&mut memo,0, 10))
}
