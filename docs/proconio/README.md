# proconio

Rustでは他の言語と比べて標準入力から値を受け取る処理を書くだけで、それなりのコード量が必要となる。
これを簡単に書くためにクレート`proconio`を使う。以下に例をC++の場合と比較して示す。


例1) 文字列、数値を標準入力から取り込む 

```C++
// 数値を受け取る
int n;
std::cin << n;

// 文字列を受け取る
std::string str;
std::cin << str;
```

```Rust
use proconio::input;

input!{
    n: i32,
    str: String,
}

```

例2) 標準入力から値の列を一次元配列として受け取る
```C++ int n;
std::cin << n;
std::vector<int> array(n);
for(std::size_t i = 0; i < n; ++i) std::cin >> array[i];
```

```Rust
use proconio::input;

input! {
    n: usize,
    array: [i32;n],
}
```

例3) 標準入力から値の列を二次元配列として受け取る

```C++
int n,m;
std::cin << n << m;
std::vector<std::vector<int>> array(n, std::vector<int>(m));

for(std::size_t i = 0; i < n; ++i){
    for(std::size_t j = 0; j < m; ++j){
        std::cin << array[i][j];
    }
}
```

```Rust
use proconio::input;

input! {
    n: usize,
    m: usize,
    array: [[i32;n]; m],
}
```
