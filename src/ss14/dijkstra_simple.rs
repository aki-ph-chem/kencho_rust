use proconio::input;

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
    let mut used = vec![false; n];
    let mut dist = vec![INF; n];
    dist[0] = 0;
    for _iter in 0..n {
        // 使用済みでない頂点うちdistが最小のものを探す
        let mut min_dist = INF;
        let mut min_v = std::usize::MAX;
        for v in 0..n {
            if !used[v] && dist[v] < min_dist {
                min_dist = dist[v];
                min_v = v
            }
        }

        // 見つからなければ終了
        if min_v == std::usize::MAX {
            break;
        }

        // min_vを始点として緩和
        for e in &graph_weight[min_v] {
            let tmp = dist[min_v] + e.weight;
            chmin(&mut dist[e.to], tmp);
        }
        // min_vを使用済みにする
        used[min_v] = true;
    }

    // 結果出力
    for v in 0..n {
        if dist[v] < INF {
            println!("{}", dist[v]);
        } else {
            println!("INF");
        }
    }
}
