use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone,Debug)]
struct Edge {
    to: usize,
    w: i64,
}

impl Edge {
    fn new(to: usize, w: i64) -> Edge {
        Edge{to, w}
    } 
}

type Graph = Vec<Vec<Edge>>;
type GraphRef = Vec<Vec<Rc<RefCell<Edge>>>>;

struct Search {
    seen: Vec<bool>,
} 

impl Search {
    fn new(n: usize) -> Search {
        Search{seen: vec![false;n]}
    }

    // for e in graph[v] とすると graph[v]に対してメソッドinto_iter()が
    // `graph[v].into_iter()` 呼び出されるため、graph[v]の値がmoveされてしまう
    // というのも`into_iter()`のシグニチャは`into_inter(&self) -> IntoIter`であるためである。
    // そのため for e in &graph[v]とする
    fn search(&mut self, graph: &Graph, ini: usize) {
        self.seen[ini] = true;
        for e in &graph[ini] {
            if self.seen[e.to] {
                continue;
            }
            println!("visit {}", e.to);
            self.search(graph, e.to);
        }
    }

    // Edgeのwを10倍しながらdfsする 
    fn search_mut(&mut self, graph: &mut Graph, ini: usize) {
        self.seen[ini] = true;
        for e in &mut graph[ini] {
            if self.seen[e.to] {
                continue;
            }
            e.w *= 10;
            println!("visit {}", e.to);
            // for e in &mut graph[ini]で可変参照しているので
            // self.search()でもう一回可変参照できない
            //
            //self.search_mut(graph, e.to);
        }
    }

    // EdgeをRc<RefCell<T>>でラップしてEdgeのフィールドの値を参照したい時に
    // borrow(), borrow_mut()を経由してアクセスする
    fn search_ref(&mut self, graph: &GraphRef, ini: usize) {
        self.seen[ini] = true;
        for e in &graph[ini] {
            if self.seen[e.borrow().to] {
                continue;
            }
            e.borrow_mut().w *= 10;
            println!("visit {}", e.borrow().to);
            self.search_ref(graph, e.borrow().to);
        }
    }
}

fn main() {
    let pair = [
        (0,1, 3),
        (1,2, 10),
        (2,0, 5),
        (1,3, 8),
    ];

    let mut graph:Graph = vec![vec![]; pair.len()];
    for (a,b,w) in pair {
        graph[a].push(Edge::new(b, w));
    }

    let mut graph_ref:GraphRef = vec![vec![]; pair.len()]; 
    for (a,b,w) in pair {
        graph_ref[a].push(Rc::new(RefCell::new(Edge::new(b,w))));
    }
    eprintln!("graph_ref = {:#?}", graph_ref);

    let mut search = Search::new(pair.len());

    //search.search(&graph, 0);
    search.search_ref(&graph_ref, 0);
    eprintln!("graph_ref = {:#?}", graph_ref);
}
