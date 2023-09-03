// バケットの最大サイズ
const MAX: usize = 100_000;

fn bucket_sort(array: &mut Vec<usize>) {
    let n = array.len();

    // 各要素をカウントするバケット
    let mut num = vec![0; MAX];
    for i in 0..n {
        num[array[i]] += 1;
    }

    // numの累積和をとる
    // sum[v]: v以下の値の個数
    // 値array[i]が全体の中で何番目に小さいかを求める
    let mut sum = vec![0; MAX];
    for v in 1..MAX {
        sum[v] = sum[v-1] + num[v];
    }

    // sum をもとにしたsort処理
    // array_2: arrayをsortしたもの
    let mut array_2 = vec![0; n];
    for i in (0..n).rev() {
        sum[array[i]] -= 1;
        array_2[sum[array[i]]] = array[i]; 
    }
    *array = array_2;
}

fn main() {
    let mut array = vec![4, 1, 3, 5, 2];
    let n = array.len();
    println!("n = {}", n);
    println!("array = {:?}", array);

    bucket_sort(&mut array);
    println!("array = {:?}", array);
}
