fn heapify(array: &mut [i32], i: usize, n: usize) {
    let mut child_1 = 2 * i + 1; // 左の子の添字
    // 子がいないときは終了
    if child_1 >= n {
        return;
    }
    // 子同士を比較
    if child_1 + 1 < n && array[child_1 + 1] > array[child_1] {
        child_1 += 1;
    }
    // 逆転がなければ終了
    if array[child_1] < array[i] {
        return;
    }
    // swap
    array.swap(i, child_1);
    // 再帰的にヒープ化
    heapify(array, child_1, n);
}

fn heap_sort(array: &mut [i32]) {
    let n = array.len();
    // array全体をヒープにする
    for i in (0..(n/2)).rev() {
        heapify(array, i, n);
    }

    // ヒープから1個1個最大値をpopする
    for i in (0..n).rev() {
        array.swap(0, i); // ヒープの最大値を右詰めにする
        heapify(array, 0, i); // サイズiのヒープ
    }
}

fn main() {
    let mut array = [4, 1, 3, 5, 2];
    let n = array.len();
    println!("n = {}", n);
    println!("array = {:?}", array);
    heap_sort(&mut array);

    println!("array = {:?}", array);
}
