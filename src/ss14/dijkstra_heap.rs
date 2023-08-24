use proconio::input;
use std::collections::binary_heap;

const INF:i64 = 1 << 60;

// エッジの定義
#[derive(Clone,Debug)]
struct Edge {
    to: usize,
    weight: i64,
}

impl Edge {
    fn new(to: usize, weight: i64) -> Edge {
        Edge{to, weight}
    }
}

// グラフの定義
type Graph = Vec<Vec<Edge>>;

// 緩和処理
fn chmin<T>(a: &mut T, b: T) -> bool 
where T: std::cmp::PartialOrd 
{
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

// (dist[v], v)を保持
struct Piar(i64, usize);

fn main() {
    //n: 頂点数 
    //m: 辺数
    //s: 始点
    input!{
        n: usize,
        m: usize,
        s: usize,
        pair: [(usize,usize, i64); m],
    }

    // 有向グラフ
    let mut graph_weight:Graph = vec![vec![]; n];
    for (a, b, w) in pair {
        graph_weight[a].push(Edge::new(b, w));
    }

    // ダイクストラ法
    let mut dist = vec![INF; n];
    dist[s] = 0;
}
