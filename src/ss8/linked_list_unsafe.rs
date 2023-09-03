struct Node<T> 
where T: std::fmt::Display
{
    item: T,
    next: *mut Node<T>,
}

impl<T> Node<T> 
where T: std::fmt::Display
{
    fn new(item: T) -> Self {
        Self {item: item, next: std::ptr::null_mut()}
    }
}

unsafe fn insert_node<T>(prev: *mut Node<T>, v: *mut Node<T>) 
    where T: std::fmt::Display
{
    (*v).next = (*prev).next; 
    (*prev).next = v;
}

unsafe fn show_list<T>(head: *mut Node<T>) 
    where T: std::fmt::Display
{
    let mut current:*mut Node<T> = (*head).next;
    while current != head {
        print!("{} ->", (*current).item);
        current = (*current).next;
    }
    println!("");
}

fn main() {
    // リストの先頭
    let mut head = Node::new("head".to_string());
    // nextは自分を指すようにする
    head.next = &mut head;

    // headに対してnode_1 ~ node_3をつなげる
    let mut node_1 = Node::new("yamamoto".to_string());
    let mut node_2 = Node::new("watanabe".to_string());
    let mut node_3 = Node::new("ito".to_string());
    unsafe {
        insert_node(&mut head, &mut node_1);
        insert_node(&mut node_1, &mut node_2);
        insert_node(&mut node_2, &mut node_3);
    }

    // リストを可視化
    unsafe {
        show_list(&mut head);
    }

    // 新たにnode_4を作ってnode_2の後ろに挿入する
    let mut node_4 = Node::new("mako".to_string());
    unsafe {
        insert_node(&mut node_2, &mut node_4);
    }

    // リストを可視化
    unsafe {
        show_list(&mut head);
    }
}
