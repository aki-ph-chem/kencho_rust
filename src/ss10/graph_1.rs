use proconio::input;

// グラフの定義
type Graph = Vec<Vec<usize>>;

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
}
