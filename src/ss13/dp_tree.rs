// 木の上でのdp
use proconio::input;

type Graph = Vec<Vec<usize>>;

// 木の上での探索の基本形
// v: 現在探索中の頂点
// p: vの親(vが根であるときはp = -1)
fn dfs(graph: &Graph, subtree_size: &mut Vec<usize>, depth: &mut Vec<usize>, v: usize, p: usize, d: usize) {
    depth[v] = d;
    for c in &graph[v] {
        // 探索が親方向へ逆流するのを防ぐ
        if *c == p {
            continue;
        }

        // cはvの各子頂点を動く
        dfs(graph, subtree_size, depth, *c, v, d + 1);
    }

    // 帰りがけに部分木のサイズを求める
    subtree_size[v] = 1;
    for c in &graph[v] {
        if *c == p {
            continue;
        }
        // 小頂点を寝とする部分木のサイズを加算する
        subtree_size[v] += subtree_size[*c];
    }
}

fn main() {
    //n: 頂点数 
    //木の辺の数は n -1
    input!{
        n: usize,
        pair: [(usize,usize); n - 1],
    }

    let mut graph: Graph= vec![vec![]; n];
    for (a, b) in pair {
        graph[a].push(b);
        graph[b].push(a);
    }
    println!("graph = {:#?}", graph);
    let mut subtree_size = vec![0; n];
    let mut depth = vec![0; n];
    dfs(&graph, &mut subtree_size, &mut depth, 0, std::usize::MAX, 0);

    for v in 0..n {
        print!("{}: depth = {} ", v, depth[v]);
        println!(", subtree_size = {}", subtree_size[v]);
    }
}
