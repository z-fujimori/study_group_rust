---
marp: true
---

# Rustを始めよう！ついでに競プロも始めよう！

---

# 本勉強会の目指すところ

- Rust面白い！と感じてもらう
- Rustを始める第一歩としてもらえたら

---

# Rustとはどんな言語か

---

# 今日の題材について
# AtCoderってなに？

---

以下早速はじめて行こう！

---

# 事前準備
- Rustのインストール
- VSCode + Rust拡張のインストール
- AtCoderのアカウント作成

---

# Hello World

```bash
cargo init
```

```rust
fn main() {
    println!("Hello, world!");
}
```

```bash
cargo run
```

---

# AtCoderについて

題材について紹介
コンテストが開かれている
問題の難易度が幅広い

---

# Rustの基本の書き方

- 静的言語(プリミティブ？型について)
    - i32, f32, u32, usize
    - String, str, chr
- 変数宣言
- for
- if
- 標準入力

---

# 変数宣言

```rust
let x = 5; // 不変な変数
let mut y = 10; // 可変な変数
println!("x:{} y:{}", x, y);

x += 1;
y += 1;
println!("x:{} y:{}", x, y);
```
(↑ フォーマットして出力するprintの書き方)

---

# 変数宣言

```bash
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:6:5
  |
2 |     let x = 5; // 不変な変数
  |         - first assignment to `x`
...
6 |     x += 1;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5; // 不変な変数
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `demo_pro` (bin "demo_pro") due to 1 previous error
```

---

# if文

```rust
let n = 10;

if n % 2 == 0 {
    println!("nは偶数");
} else {
    println!("nは奇数");
}
```

---

# for文

```rust
let arr = [1, 2, 3, 4, 5]; // 配列(固定長)

for i in 0..5 {
    println!("{}", arr[i]);
}
```

---

# 標準入力

```rust
use std::io::input;

input! {
    n: usize,
    arr: [i32; n],
}

for i in 0..5 {
    println!("{}", arr[i]);
}
```
```input.txt
5
1 2 3 4 5
```
```bash
cargo run < input.txt
```

--- 

# 問題を解いてみよう！
mut, if, for

AtCoder ABCコンテスト 330 A問題
https://atcoder.jp/contests/abc330/tasks/abc330_a

---

回答/別解(for)

---

# 文字列操作に挑戦！
所有権, 借用, String

AtCoder ABCコンテスト 149 A問題
https://atcoder.jp/contests/abc149/tasks/abc149_a

---

回答/別解

---

# RustのStringについて

---



