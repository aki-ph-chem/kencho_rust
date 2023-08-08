use proconio::input;

// グラフの定義
type Graph = Vec<Vec<usize>>;

fn dfs(graph: &Graph, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    // vから行ける頂点を探索
    for next_v in &graph[v] {
        if seen[*next_v] {
            continue;
        }
        println!("now vist: {}", next_v);
        dfs(graph, seen, *next_v);
    }
}

fn main() {
    //n: 頂点数 
    //m: 辺数
    input!{
        n: usize,
        m: usize,
        pair: [(usize,usize); m],
    }

    // 有向グラフ
    let mut graph: Graph= vec![vec![]; n];
    for (a, b) in pair {
        graph[a].push(b);
    }
    println!("graph = {:#?}", graph);

    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        dfs(&graph, &mut seen, i);
    }
}
