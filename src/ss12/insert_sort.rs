fn insert_sort(array: &mut [i32]) {
    let n = array.len();
    for i in 1..n {
        let v = array[i]; //挿入したい値
        // vを挿入する場所を探す
        let mut j = i;
        while j > 0 {
            if array[j - 1] > v {
                // vよりも大きいものは一つ後ろに移す
                array[j] = array[j - 1];
            } else {
                // v以下になったら停止する
                break;
            }
            j -= 1;
        }
        array[j] = v; 
    }
}

fn main() {
    let mut array = [4, 1, 3, 5, 2];
    println!("array = {:?}", array);
    insert_sort(&mut array);
    println!("array = {:?}", array);
}
