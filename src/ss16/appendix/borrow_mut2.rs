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

struct Graph {
    list: Vec<Vec<Rc<RefCell<Edge>>>>,
}

impl Graph {
    fn inc_w(&self, e: &Rc<RefCell<Edge>>, d: i64) {
        e.borrow_mut().w += d;
    }
}

fn my_search(seen: &mut Vec<bool>, graph: &mut Graph, ini: usize) {
    seen[ini] = true;
    for e in &graph.list[ini] {
        e.borrow_mut().w *= 10;
        println!("visit {}", e.borrow().to);
        graph.inc_w(e, 1000);
    }
}

fn main() {
}
