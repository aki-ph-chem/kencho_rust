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

    //ベルマン・フォード法
    let mut exist_negative_cycle = false;
    let mut dist = vec![INF; n];
    dist[s] = 0;
    for iter in 0..n {
        let mut update = false;
        for v in 0..n {
            // dist[v] = INFのときは頂点vから緩和を行わない
            if dist[v] == INF {
                continue;
            }
            for e in &graph_weight[v] {
                // 緩和処理を行い、更新されたらupdateをtrueに変更
                let dist_tmp = dist[v] + e.weight;
                if chmin(&mut dist[e.to], dist_tmp) {
                    update = true;
                }
            }
        }
        // 更新が行われなかったら、最短経路はすでに求められている
        if !update {
            break;
        }

        // n回目の更新が行われたならば、負閉路を持つ
        if iter == n - 1 && update {
            exist_negative_cycle = true;
        }
    }

    // 結果の出力
    if exist_negative_cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for v in 0..n {
            if dist[v] < INF {
                println!("{}",dist[v]);
            } else {
                println!("INF");
            }
        }
    }
}
