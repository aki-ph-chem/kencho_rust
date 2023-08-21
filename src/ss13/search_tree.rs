use proconio::input;

type Graph = Vec<Vec<usize>>;

// 根なし木の走査
// v: 現在探索中の頂点
// p: vの親、vが根であるときは p = -1
fn dfs_tree(graph: &Graph, v: usize, p: usize) {
    for c in &graph[v] {
        if *c == p {
            continue;
        }
        println!("now visit: {}", *c);
        // cはvの各頂点を動く
        dfs_tree(graph, *c, v);
    }
}

// 根なし木を根付き木とした時の各頂点の深さ
// v: 現在探索中の頂点
// p: vの親、vが根であるときは p = -1
// d: 頂点vの深さ
fn dfs_depth(graph: &Graph, depth: &mut Vec<usize>, v: usize, p: usize, d: usize) {
    depth[v] = d;
    for c in &graph[v] {
        if *c == p {
            continue;
        }
        dfs_depth(graph, depth, *c , v, d + 1);
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
        graph[b].push(a);
    }
    println!("graph = {:#?}", graph);

    println!("dfs_tree");
    dfs_tree(&graph, 0, std::usize::MAX);

    println!("dfs_depth");
    let mut depth = vec![0; n];
    dfs_depth(&graph, &mut depth, 0, std::usize::MAX, 0);
    println!("depth = {:#?}", depth);
}
