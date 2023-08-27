use std::collections::BinaryHeap;
use std::cmp::Ordering;

// 特にカスタムせずにPartialOrdを実装すると
// 第一フィールドの大小関係がstd::collections::BinaryHeapに使われる
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Coordinate2D {
    x: i32,
    y: i32,
}

impl Coordinate2D {
    fn new(x: i32, y: i32) -> Coordinate2D {
        Coordinate2D{x,y}
    }
}

#[derive(Eq, PartialEq, Ord, Debug)]
struct Coordinate3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate3D {
    fn new(x: i32, y: i32, z: i32) -> Coordinate3D {
        Coordinate3D{x, y, z}
    }
}

// PartialOrdを第二フィールドを対象とする
impl PartialOrd for Coordinate3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.y.cmp(&other.y))
    }
} 

fn main() {
    let array = [3, 17, 10, 5, 8, 1];
    let mut h_3d = BinaryHeap::<Coordinate3D>::new();
    for value in &array {
        h_3d.push(Coordinate3D::new(*value, 2 * (-*value),3 * (*value) + 2));
    }

    let mut h_2d = BinaryHeap::<Coordinate2D>::new();
    for value in &array {
        h_2d.push(Coordinate2D::new(*value, 2 * (-*value) + 3));
    }

    while !h_3d.is_empty() {
        if let Some(v) = h_3d.peek() {
            println!("(x, y, z) = ({}, {}, {})", v.x, v.y, v.z);
        }
        h_3d.pop();
    }

    while !h_2d.is_empty() {
        if let Some(v) = h_2d.peek() {
            println!("(x, y) = ({}, {})", v.x, v.y);
        }
        h_2d.pop();
    }
}
