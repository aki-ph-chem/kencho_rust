use proconio::input;

// グラフの定義
type Graph = Vec<Vec<usize>>;

fn dfs(graph: &Graph, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;

    for next_v in &graph[v] {
        if seen[*next_v] {
            continue;
        }
        dfs(graph, seen, *next_v);
    }
}

fn main() {
    //n: 頂点数 
    //m: 辺数
    input!{
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        pair: [(usize,usize); m],
    }

    // 有向グラフ
    let mut graph: Graph= vec![vec![]; n];
    for (a, b) in pair {
        graph[a].push(b);
    }
    println!("graph = {:#?}", graph);

    let mut seen = vec![false; n];
    dfs(&graph, &mut seen, s);

    // tに辿りつけるか否か
    if seen[t] {
        println!("Yese");
    } else {
        println!("No");
    }
}
