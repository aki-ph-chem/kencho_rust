fn main() {
    // 配列: 固定サイズ
    let mut array_static = [1,2,3];
    println!("array_static = {:?}", array_static);
    array_static[1] = 10;
    println!("array_static = {:?}", array_static);

    // vector: 可変サイズ
    let mut array_variable = vec![1,2,3,4];
    println!("array_variable = {:?}", array_variable);
    array_variable[2] = 33;
    array_variable.push(8);
    println!("array_variable = {:?}", array_variable);
}
