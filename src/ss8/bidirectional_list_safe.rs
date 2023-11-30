#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub prev: Option<Box<ListNode>>,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode{val, prev: None, next: None}
    }
}

// まだ途中
pub fn insert_nodde(prev: &mut ListNode, v: ListNode) {
    prev.next = Some(Box::new(v));
}

pub fn show_list_ref(head: Option<Box<ListNode>>) {
    let mut node_current = head;

    while let Some(node) = node_current {
        print!("{} -> ", node.val);
        node_current = node.next;
    }
    println!("");
}


fn main() {
    let mut l_1_0 = ListNode::new(11);
    let mut l_1_1 = ListNode::new(22);
    let mut l_1_2 = ListNode::new(33);
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));

    let ref_to_l_1_0 = Some(Box::new(l_1_0));
    show_list_ref(ref_to_l_1_0.clone());
}
