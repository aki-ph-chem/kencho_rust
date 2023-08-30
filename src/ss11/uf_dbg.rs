use std::mem;

struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    // コンストラクタ
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: vec![std::usize::MAX; n],
            size: vec![1; n],
        }
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
        let (mut x_root, mut y_root) = (self.root(x), self.root(y));
        if x_root == y_root {
            false
        } else {
            if self.size(x_root) < self.size(y_root) {
                mem::swap(&mut x_root, &mut y_root);
            }
            self.parents[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            true
        }
    }
}

fn main() {
    let mut uf = UnionFind::new(5); //{0},{1},{2},{3},{4},{5}
    uf.unite(3, 0); // {0,3},{1},{2},{4},{5}
    uf.unite(3, 4); // {0,3,4},{1},{2},{5}
    uf.unite(2, 3); // {0,3,4,2},{1},{5}
    uf.unite(0, 1); // {0,1,3,4,2},{5} // ここでstack over flowする
    uf.unite(1, 2);
}
