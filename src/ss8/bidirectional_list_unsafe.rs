struct Node<T> 
where T: std::fmt::Display
{
    item: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T>
where T: std::fmt::Display
{
    fn new(item: T) -> Self {
        Self {
            item, 
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }
    }
}

unsafe fn insert_node<T>(prev_node: *mut Node<T>, v_node: *mut Node<T>)
    where T: std::fmt::Display
{
    (*v_node).next = (*prev_node).next;
    (*(*prev_node).next).prev = v_node;
    (*prev_node).next = v_node;
    (*v_node).prev = prev_node;
}

unsafe fn show_list<T>(head: *mut Node<T>) 
    where T: std::fmt::Display
{
    let mut current = (*head).next;
    while current != head {
        print!("{} -> ", (*current).item);
        current = (*current).next;
    }
    println!("");
}

unsafe fn erase_node<T>(v_node: *mut Node<T>, head: *mut Node<T>)
    where T: std::fmt::Display 
{
    if v_node == head {
        return;
    }
    // あくまで、リストから除外するだけで、メモリの開放はしない
    unsafe {
        (*(*v_node).prev).next = (*v_node).next; 
        (*(*v_node).next).prev = (*v_node).prev; 
    }
}


fn main() {
    // リストの先頭
    let mut head = Node::new("head".to_string());
    // prev, nextが自分を指すようにする
    head.prev = &mut head;
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

    unsafe {
        show_list(&mut head);
    }

    // node_2の後ろにnode_4を挿入 
    let mut node_4 = Node::new("yukawa".to_string());
    unsafe {
        insert_node(&mut node_2, &mut node_4);
        show_list(&mut head);
    }

    // node_2を削除
    unsafe {
        erase_node(&mut node_2, &mut head);
        show_list(&mut head);
    }
}
