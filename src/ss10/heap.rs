use proconio::input;

struct Heap {
    heap: Vec<i32>,
}

impl Heap {
    fn new() -> Heap {
        Heap{heap: vec![]}
    }

    fn push(&mut self, x: i32) {
        self.heap.push(x);
        // xを挿入した位置の添字
        let mut i = self.heap.len() - 1;
        while i > 0 {
            // xの親の添字
            let p = (i - 1) / 2;
            // 親の値の方が大きければ修了
            if self.heap[p] >= x {
                break;
            }
            // 自分の値を親の値に変更 
            self.heap[i] = self.heap[p];
            // 自分は上に行く
            i = p;
        }
        self.heap[i] = x;
    }

    fn top(&self) -> i32 {
        1
    }

    fn pop(&mut self) {
    }
}

fn main() {
}
