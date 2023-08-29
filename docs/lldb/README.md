<!-- ドラフト -->
# lldbを使ったデバッグ

C/C++をgcc,g++でビルドした実行ファイルをデバッグする場合`gdb(GNU Debugger)`を用いてデバッグを行う。
一方,C/C++でもclang,clang++でビルドした場合ではデバッガとしては`lldb`を用いる。

基本的の`gdb`と同じような操作・コマンドでデバッグを行うことが可能である

以下に簡単にメモしておく

## 基本的な使い方

1. まずビルドを行う

cargoでビルドを行っているのならば、普通のビルド(debugビルド)でビルドを行う
```bash
$ carbo build
```
rustcでビルドを行っているのならば(cargoを使っていないならば)ビルドは 

```bash
$rustc -g -o <program name> <src name>.rs
```

で行う

2. `lldb`の実行

以下のコマンドで`lldb`を起動する

```bash
$ lldb <path to binary>
```

lldbが起動すると以下のようなプロンプトが表示される

```text
(lldb)
```

まず、ブレークポイントをmain()に対してい付ける

```text
(lldb) b main
```

ブレークポイントを付けたら`run`もしくは`r`コマンドで1行ずつコマンドを実行する

```text
(lldb) run 
```

終了した場合は`q`コマンドを用いる

## 関数がネストされている場合

以下のようにmain()関数から他の関数が呼び出されている場合を考える

```Rust
fn sum(n: i32) {
    println!("Inside sum()");
    let mut sum = 0;
    for i in 0..(n+1) {
        sum += i;
    }
    println!("sum = {}", sum);
}

fn fact(n :i32) {
    println!("Inside fact()");
    if n == 0{
        println!("fact = 1");
        return;
    } 
    let mut fact = 1;
    for i in 1..(n+1) {
        fact *= i;
    }
    println!("fact = {}",fact);
}

fn main() {
    sum(3);
    fact(3);
}
```

このプログラムをmain()関数の中の処理のみならず、sum()やfact()の関数の中に入ってデバッグしたいとする。

このときはプログラムカウンタ(次に実行されアドレスを指すレジスタ)がsum()もしくはfact()を指しているときに`step`コマンドを
実行することで実現できる。
