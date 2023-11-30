// Box<T>,とOption<T>を使えばunsafeなしで単方向Listを実装できる
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn insert_node(prev: &mut ListNode, v: ListNode) {
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
    let mut l_1_0 = ListNode::new(3);
    let mut l_1_1 = ListNode::new(2);
    let mut l_1_2 = ListNode::new(3);
    let mut l_1_3 = ListNode::new(4);
    let l_1_4 = ListNode::new(5);

    l_1_3.next = Some(Box::new(l_1_4));
    l_1_2.next = Some(Box::new(l_1_3));
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));

    let ref_to_l_1_0 = Some(Box::new(l_1_0));
    show_list_ref(ref_to_l_1_0.clone());

    // insert_node()を使ってListを構築する
    let mut l_2_0 = ListNode::new(111);
    let mut l_2_1 = ListNode::new(222);
    let mut l_2_2 = ListNode::new(333);
    let l_2_3 = ListNode::new(444);
    insert_node(&mut l_2_2, l_2_3);
    insert_node(&mut l_2_1, l_2_2);
    insert_node(&mut l_2_0, l_2_1);

    let ref_to_l_2_0 = Some(Box::new(l_2_0));
    show_list_ref(ref_to_l_2_0.clone());
}
