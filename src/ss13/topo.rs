use proconio::input;

type Graph = Vec<Vec<usize>>;

fn rec(graph: &Graph, seen: &mut Vec<bool>, order: &mut Vec<usize>, v: usize) {
    seen[v] = true;
    for next_v in &graph[v] {
        if seen[*next_v] {
            continue;
        }
        rec(graph, seen, order, *next_v);
    }
    // v-outの記録
    order.push(v);
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
    let mut order = vec![];

    for v in 0..n {
        if seen[v] {
            continue;
        }
        rec(&graph, &mut seen, &mut order, v);
    }

    order.reverse();
    for v in order {
        print!("{} -> ", v);
    }
    println!();
}
