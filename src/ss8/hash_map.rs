use std::collections::hash_map::HashMap;

fn main() {
    let mut m:HashMap<String, i32> = HashMap::new();
    m.insert("hoge".to_string(), 11);
    m.insert("fuga".to_string(), 31);

    println!("m = {:?}", m);
}
