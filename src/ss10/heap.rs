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

    // 最大値を知る
    fn top(&self) -> i32 {
        if !self.heap.is_empty() {
            self.heap[0]
        } else {
            -1
        }
    }

    // 最大値を削除
    fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        }
        // 頂点に持ってくる値
        let x = self.heap.iter()
            .rev()
            .next()
            .unwrap()
            .clone();
        self.heap.pop();
        // 根から下ろしていく
        let mut i = 0;
        while i * 2 + 1 < self.heap.len() {
            // 子の値を比較して大きい方をchild_1とする
            let mut child_1 = i * 2 + 1;
            let child_2 = i * 2 + 2;
            if child_2 < self.heap.len() 
                && self.heap[child_2] > self.heap[child_1] {
                    child_1 = child_2;
            }
            // 逆転がなければ終了
            if self.heap[child_1] <= x {
                break;
            }
            // 自分の値を子の値にする
            self.heap[i] = self.heap[child_1];
            // 自分は下に行く
            i = child_1;
        }
        // xを最終的にここにもっていく
        self.heap[i] = x;
    }
}

fn main() {
    let mut h = Heap::new();
    h.push(5); h.push(3); h.push(7); h.push(1);

    println!("h.top() = {}", h.top()); // 7
    h.pop();

    println!("h.top() = {}", h.top()); // 5 
    h.pop();

    println!("h.top() = {}", h.top()); // 11 
    h.pop();
}
