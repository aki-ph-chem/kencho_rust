struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    // コンストラクタ
    fn new(n: usize) -> UnionFind {
        UnionFind{parents: vec![std::usize::MAX; n], size: vec![1; n]}
    }

    // xを含むグループのサイズ
    fn size(&self, x: usize) -> usize {
        self.size[x]
    }

    // 経路圧縮なしの場合の実装
    // xが根のときはparents[x] = std::usize::MAX
    fn root_naive(&self, x: usize) -> usize {
        if self.parents[x] == std::usize::MAX {
            x
        } else {
            self.root_naive(self.parents[x])
        }
    }

    // 経路圧縮ありの場合の実装
    fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == std::usize::MAX {
            x
        } else {
            self.parents[x] = self.root(self.parents[x]);
            self.parents[x]
        } 
    }

    // 頂点x,yが同じグループに属するか否か
    // 根が同じが判定すれば良い
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xを含むグループとyを含むグループを併合する
    fn unite(&mut self, x: usize, y: usize) -> bool {
        let (x_root, y_root) = (self.root(x), self.root(y));
        if x_root == y_root {
            false
        } else {
            let x_root = if self.size(x_root) < self.size(y_root) {
                y_root
            } else {
                x_root
            };
            self.parents[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            true
        }
    }
}

fn main() {
    let mut uf = UnionFind::new(7); // {0},{1},{2},{3},{4},{5},{6}

    uf.unite(1, 2); // {0}, {1, 2}, {3}, {4},{5},{6}
    uf.unite(2, 3); // {0}, {1, 2, 3},{4},{5},{6}
    uf.unite(5, 6); // {0}, {1, 2, 3},{4},{5,6}
    println!("uf.is_same(1,3) = {}", uf.is_same(1, 3)); // true
    println!("uf.is_same(1,2) = {}", uf.is_same(1, 2)); // true
    println!("uf.is_same(2,3) = {}", uf.is_same(2, 3)); // true
    println!("uf.is_same(0,3) = {}", uf.is_same(0, 3)); // false
    println!("uf.is_same(2,5) = {}", uf.is_same(2, 5)); // false

    uf.unite(1, 6); // {0}, {1, 2, 3, 5, 6},{4}
    println!("uf.is_same(2,5) = {}", uf.is_same(2, 5)); // true
    println!("uf.is_same(3,5) = {}", uf.is_same(3, 5)); // true
}
