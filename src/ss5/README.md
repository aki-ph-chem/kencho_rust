<!-- ドラフト -->
# 動的計画法

## Frog

## ナップサック問題

## 編集距離

- dp[i][j]: Sの $i$ 文字とTの $j$ 文字との間との間の編集距離
    - 初期条件: dp[0][0] = 0

配列dpを更新していく過程において $(i,j)$ に至る過程は 
- 右下への移動( $(i-1,j-1)$ -> $(i,j)$ ): Sの文字の $i - 1$ 文字目とTの $j - 1$文字を比較して等しければコスト0,等しくなければコスト1 
```Rust
let tmp = dp[i - 1][j - 1] + 1;
chmin(&mut dp[i][j], tmp);
```
- 下への移動( $(i-1,j)$ -> $(i,j)$ ): Sの $i$ 文字目の削除

```Rust
let tmp = dp[i - 1][j] + 1;
chmin(&mut dp[i][j], tmp);
```
- 右への移動( $(i,j-1)$ -> $(i,j)$ ): Sへの挿入(Tの $j$ 文字目の削除)

```Rust
let tmp = dp[i][j - 1] + 1;
chmin(&mut dp[i][j], tmp);
```

の三種類がある
