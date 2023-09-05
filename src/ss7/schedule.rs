use proconio::input;
use std::cmp::Ordering;

#[derive(Debug, Ord, Eq, PartialEq)]
struct Interval(i32, i32);

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

fn main() {
    input! {
        n: i32,
        inter: [(i32, i32); n],
    }
    let mut interval = vec![];
    for (a, b) in inter {
        interval.push(Interval(a, b));
    }
    // 区間をsort
    interval.sort();
    //eprintln!("interval: {:#?}", interval);

    let mut result = 0;
    let mut current = 0;
    for s in interval {
        if s.0 < current {
            continue;
        }
        result += 1;
        current = s.1;
    }
    println!("{}", result);
}
