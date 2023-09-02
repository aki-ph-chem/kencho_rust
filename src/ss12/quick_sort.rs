fn quick_sort(array: &mut [i32], left: usize, right: usize) {
    if right - left <= 1 {
        return;
    }

    let pivot_index = (left + right) / 2;
    let pivot = array[pivot_index];
    // pivotと右端をswap
    array.swap(pivot_index, right - 1);

    let mut i = left;
    // pivot未満を左に詰める
    for j in left..(right - 1) {
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    // pivotを適当な場所に挿入
    array.swap(i, right - 1);

    // 再帰的に解く
    quick_sort(array, left, i);
    quick_sort(array, i + 1, right);
}

fn main() {
    let mut array = [4, 1, 3, 5, 2];
    let n = array.len();
    println!("n = {}", n);
    println!("array = {:?}", array);
    quick_sort(&mut array, 0, n);

    println!("array = {:?}", array);
}
