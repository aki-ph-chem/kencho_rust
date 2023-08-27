use std::collections::BinaryHeap;

// C++のstd::pairの代用
struct Person(String,i32);

impl Person {
    fn show(&self) {
        println!("name: {}, age: {}", self.0, self.1);
    }
}

fn main() {
    let p_0 = Person("Hoge".to_string(), 112);
    p_0.show();

    let array = [1,2,3,4];
    let mut h_0 =BinaryHeap::<i32>::new(); 

    for x in array {
        h_0.push(x);
    }

    println!("h_0");
    while !h_0.is_empty() {
        if let Some(v) = h_0.peek() {
            println!("h_0 = {}",v);
        }
        h_0.pop();
    }
}
