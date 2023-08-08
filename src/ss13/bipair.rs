use proconio::input;

// グラフの定義
type Graph = Vec<Vec<usize>>;

// 二部グラフの判定
fn dfs(graph: &Graph, color: &mut Vec<i32>, v: usize, cur: i32) -> bool {
    color[v] = cur;
    for next_v in &graph[v] {
        // 隣接する頂点がすでに色確定していた場合
        if color[*next_v] != -1 {
            // 同じ色が隣接したならば二部グラフではない
            if color[*next_v] == cur {
                return false;
            }

            // 色が確定した場合はスキップ
            continue;
        }
        // 隣接頂点の色を変えて再帰的に探索
        if !dfs(graph, color, *next_v, 1 - cur) {
            return false;
        }
    }
    true
}

fn main() {
    //n: 頂点数
    //m: 辺数
    input! {
        n: usize,
        m: usize,
        pair: [(usize,usize); m],
    }

    // 無向グラフ
    let mut graph: Graph = vec![vec![]; n];
    for (a, b) in pair {
        graph[a].push(b);
        graph[b].push(a);
    }
    println!("graph = {:#?}", graph);

    let mut color = vec![-1; n];
    let mut is_bipartite = true;
    for v in 0..n {
        if color[v] != -1 {
            continue;
        }

        if !dfs(&graph, &mut color, v, 0) {
            is_bipartite = false;
        }
    }

    if is_bipartite {
        println!("Yes");
    } else {
        println!("No");
    }
}
