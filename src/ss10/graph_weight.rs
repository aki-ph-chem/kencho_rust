use proconio::input;

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

fn main() {

    //n: 頂点数 
    //m: 辺数
    input!{
        n: usize,
        m: usize,
        pair: [(usize,usize, i64); m],
    }

    let mut graph_weight:Graph = vec![vec![]; n];
    for (a, b, w) in pair {
        graph_weight[a].push(Edge::new(b, w));
    }

    println!("graph_weight = {:#?}", graph_weight);
}
