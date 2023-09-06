use std::ops::{Index,IndexMut};
use std::cmp::min;
use std::rc::Rc;
use std::cell::RefCell;

const INF: i32 = 1 << 30;

#[derive(Clone)]
struct Edge {
    rev: usize,
    from: usize,
    to: usize,
    cap: i32,
}

impl Edge {
    fn new(rev: usize, from: usize, to: usize, cap: i32) -> Edge {
        Edge{rev, from, to, cap}
    }
}

struct Graph {
    // 近接リスト
    list: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Graph {
        Graph {list: vec![vec![];n]}
    }

    // グラフの頂点数の取得
    fn size(&self) -> usize {
        self.list.len()
    }

    // 辺 e = (u,v)の逆辺 (v,u)を取得する
    fn rev_edge(&mut self, e: &mut Edge) -> &mut Edge {
        &mut self.list[e.to][e.rev]
    }

    // 辺 e = (u,v)に流量fのflowを流す
    // このとき、(u,v)の流量はfだけ現象する
    // 逆辺 (v,u)の流量はfだけ増加する 
    fn run_flow(&mut self, e: &mut Edge, f: i32) {
        e.cap -= f;
        self.rev_edge(e).cap += f;
    }

    // 頂点 from からtoへ容量capの辺を張る
    // このときtoからfromへ容量0の辺を張る
    fn add_edge(&mut self, from: usize, to: usize, cap: i32) {
        let from_rev = self.list[from].len();
        let to_rev = self.list[to].len();
        self.list[from].push(Edge::new(to_rev, from, to, cap));
        self.list[from].push(Edge::new(from_rev, to, from, 0));
    }
}


// [] をオーバーロード
// 不変
impl Index<usize> for Graph {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

// 可変
impl IndexMut<usize> for Graph {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Edge> {
        &mut self.list[index]
    }
}

struct FordFulkerson {
    seen: Vec<bool>,
}

impl FordFulkerson {
    fn new() -> FordFulkerson {
        FordFulkerson{seen: vec![]}
    }

    // 残余グラフ上でs-t path を DFSで探す
    // 返り値は s-t path上での容量の最小値
    // f: sからvへ到達した過程の各辺の容量の最小値
    fn ford_fulkerson(&mut self, graph: Rc<RefCell<Graph>>, v: usize, t: usize, f: i32) -> i32 {
        // 終端tに到達したらreturn
        if v == t {
            return f;
        }
        // DFS
        self.seen[v] = true;
        for mut e in graph.borrow_mut()[v]{
            if self.seen[e.to] {continue;}
            // 容量0の辺は存在しない(ことになっている)
            if e.cap == 0 {
                continue;
            }
            // s-t pathを探す
            // 見つかった場合flowはpathの最小容量
            // 見つからなかった場合 f = 0
            let flow = self.ford_fulkerson(graph, e.to, t, min(f, e.cap)); 
            // s-t pathが見つからなければ次の辺を試す
            if flow == 0 {continue;}
            // eに容量flowのフローを流す
            graph.borrow_mut().run_flow(&mut e, flow);
            // s-t パスが見つかったらpath上の最小容量を返す
            return flow;
        }
        // s-t path が見つからなかった
        0
    }
}


fn main() {
}
