use proconio::input;
use std::cmp::Ordering;

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
        let (x_root, y_root) = (self.root(x), self.root(y));
        if x_root == y_root {
            false
        } else {
            let (x_root, y_root) = if self.size(x_root) < self.size(y_root) {
                (y_root, x_root)
            } else {
                (x_root, y_root)
            };
            self.parents[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            true
        }
    }
}

// 辺 e = (u,v)を {w(e), (u,v)}で表現する
#[derive(Clone, Eq, PartialEq, Ord, Debug)]
struct Edge {
    weight: i64,
    e: (usize, usize),
}

// Edgeの比較はweightで行う
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Edge {
    fn new(weight: i64, e: (usize, usize)) -> Edge {
        Edge { weight, e }
    }
}

fn main() {
    //n: 頂点数
    //m: 辺数
    input! {
        n: usize,
        m: usize,
        pair: [(usize,usize, i64); m],
    }
    // 辺の集合
    let mut edges = vec![Edge::new(0, (0, 0)); m];
    for (i, (u, v, w)) in pair.iter().enumerate() {
        edges[i] = Edge::new(*w, (*u, *v));
    }

    // 各辺を重みでsortする
    edges.sort();
    eprintln!("edges = {:#?}", edges);

    // クラスカル法
    let mut res = 0;
    let mut uf = UnionFind::new(n);
    for i in 0..m {
        let w = edges[i].weight;
        let (u, v) = edges[i].e;
        // 辺 (u,v) の追加によってサイクルが形成されるときは追加をしない
        if uf.is_same(u, v) {
            continue;
        }
        // 辺 (u,v) を追加する
        res += w;
        uf.unite(u, v);
    }
    // 結果の表示
    println!("{}", res);
}
