use proconio::input;

// グラフの定義
type Graph = Vec<Vec<usize>>;

fn main() {
    input!{
        n: usize,
        m: usize,
        pair: [(usize,usize); m],
    }

    // 無向グラフ
    let mut graph: Graph= vec![vec![]; n];
    for (a, b) in pair {
        graph[a].push(b);
        graph[b].push(a);
    }

    println!("graph = {:#?}", graph);
}
