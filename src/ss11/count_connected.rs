use proconio::input;

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
    //n: 頂点数, m: 辺数
    input!{
        n: usize,
        m: usize,
        pair: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);

    for (a,b) in pair {
        uf.unite(a, b);
    }

    // 集計
    let mut sum = 0;
    for x in 0..n {
        if uf.root(x) == x {
            sum += 1;
        }
    }
    println!("sum = {}", sum);
}
