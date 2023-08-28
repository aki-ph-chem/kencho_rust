use proconio::input;
use std::collections::BinaryHeap;

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
#[derive(Eq, Ord, PartialEq, PartialOrd)]
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
    let mut que = BinaryHeap::new();
    que.push(Piar(dist[s], s));
    // ダイクストラ法の反復
    while !que.is_empty() {
        //v: 使用済みでない頂点のうちd[v]お最小の頂点
        let v = que.peek().unwrap().1;
        let d = que.peek().unwrap().0;
        que.pop();
        // dがゴミ化否か判定
        if d > dist[v] {
            continue;
        }
        for e in &graph_weight[v] {
            // 更新があればヒープに新たに挿入
            let tmp = dist[v] + e.weight;
            if chmin(&mut dist[e.to], tmp) {
                que.push(Piar(dist[e.to], e.to));
            }
        }
    }
    // 結果の出力
    for v in 0..n {
        if dist[v] < INF {
            println!("{}", dist[v]);
        } else {
            println!("INF");
        }
    }
}
