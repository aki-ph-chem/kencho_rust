// 木の上でのdp
use proconio::input;

type Graph = Vec<Vec<usize>>;

fn dfs(graph: &Graph, v: usize, p: usize) {
    for c in &graph[v] {
        // 探索が親方向へ逆流するのを防ぐ
        if *c == p {
            continue;
        }

        // cはvの各子頂点を動く
        dfs(graph, *c, v);
    }
}

fn main() {
    //n: 頂点数 
    //木の辺の数は n -1
    input!{
        n: usize,
        pair: [(usize,usize); n - 1],
    }

    // 有向グラフ
    let mut graph: Graph= vec![vec![]; n];
    for (a, b) in pair {
        graph[a].push(b);
    }
    println!("graph = {:#?}", graph);

}
