fn merge_sort(array: &mut [i32], left: usize, right: usize) {
    if right - left == 1 {
        return;
    }

    let mid = (left + right) / 2;
    // 左半分をsort
    merge_sort(array, left, mid);
    // 右半分をsort
    merge_sort(array, mid, right);

    // 一時的に左側、右側をバッファする
    let mut buf:Vec<i32> = vec![]; 
    for i in left..mid {
        buf.push(array[i]);
    }
    for j in (mid..right).rev() {
        buf.push(array[j]);
    } 

    // 併合
    let mut index_left = 0;
    let mut index_right = buf.len() - 1;
    for i in left..right {
        // 左側
        if buf[index_left] <= buf[index_right] {
            index_left += 1;
            array[i] = buf[index_left];
        } 
        // 右側
        else {
            index_right -= 1;
            array[i] = buf[index_right];
        }
    }
}

fn main() {
    let mut array = [4, 1, 3, 5, 2];
    let n = array.len();
    println!("n = {}", n);
    println!("array = {:?}", array);

    merge_sort(&mut array, 0, n);
    println!("array = {:?}", array);
}
