use proconio::input;
use std::collections::vec_deque::VecDeque;

// グラフの定義
type Graph = Vec<Vec<usize>>;

fn bfs(graph: &Graph, s: usize) {
    let length_graph = graph.len();  
    let mut seen = vec![false; length_graph];  
    let mut todo = VecDeque::new();

    // 初期条件
    seen[s] = true;
    // スタート地点を訪問済みにする
    todo.push_back(s);

    // todoが空になるまで探索
    while !todo.is_empty() {
        let v = todo.front().unwrap().clone(); 
        todo.pop_front();

        // vから辿れる頂点を探索
        for x in &graph[v] {
            if seen[*x] {
                continue;
            } 

            println!("now visit {}", x);
            // xを訪問済みにする
            seen[*x] = true;
            todo.push_back(*x);
        }
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

    bfs(&graph, 0);
}
