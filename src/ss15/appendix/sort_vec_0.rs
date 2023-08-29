use std::cmp::Ordering;

#[derive(Eq, PartialEq, Ord, Debug)]
struct Pair {
    x: i32,
    y: i32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.x.cmp(&other.x))
    }
}

fn main() {
    let mut vec_0 = vec![4, 10, 2, 3, 8];
    vec_0.sort();
    for (index, value) in vec_0.iter().enumerate() {
        println!("vec_0[{}] = {}", index, value);
    }

    let mut vec_pair = vec![
        Pair { x: 1, y: 5 },
        Pair { x: 8, y: 3 },
        Pair { x: 2, y: 18 },
        Pair { x: 3, y: 3 },
    ];
    vec_pair.sort();

    for v in &vec_pair {
        println!("v = {:#?}", *v);
    }
}
