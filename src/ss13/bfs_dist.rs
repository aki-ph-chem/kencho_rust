use proconio::input;
use std::collections::vec_deque::VecDeque;

// グラフの定義
type Graph = Vec<Vec<usize>>;

fn bfs(graph: &Graph, s: usize) -> Vec<i32> {
    let length_graph = graph.len();  
    let mut dist = vec![-1; length_graph];
    let mut todo = VecDeque::new();

    // 初期条件
    dist[s] = 0;
    // スタート地点を訪問済みにする
    todo.push_back(s);

    // todoが空になるまで探索
    while !todo.is_empty() {
        let v = todo.front().unwrap().clone(); 
        todo.pop_front();

        // vから辿れる頂点を探索
        for x in &graph[v] {
            if dist[*x] != -1 {
                continue;
            } 
            dist[*x] = dist[v] + 1; 
            todo.push_back(*x);
        }
    }
    dist
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

    let dist = bfs(&graph, 0);
    println!("dist = {:#?}", dist);
}
