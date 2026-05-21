# Rustの文字列型

## 概要

Rustには主に2種類の文字列型がある。

| 型 | 種別 | メモリ | 変更可否 |
|---|---|---|---|
| `&str` | 文字列スライス（プリミティブ） | スタック or データセグメント | 不変 |
| `String` | ヒープ上の文字列（標準ライブラリ） | ヒープ | 可変 |

どちらも **UTF-8エンコード** で文字列を表す。

---

## `&str`（文字列スライス）

文字列データへの参照。実態はバイト列 `[u8]` へのスライス。

```rust
let s: &str = "hello"; // 文字列リテラル。データセグメントに埋め込まれる
```

### 特徴

- サイズがコンパイル時に不明なため、常に参照（`&str`）として扱う
- 文字列リテラルは `'static` ライフタイムを持つ（プログラム全体で有効）
- `String` のスライスも `&str` として参照できる

```rust
let owned: String = String::from("hello");
let slice: &str = &owned[0..3]; // "hel"
let whole: &str = &owned;       // "hello"
```

---

## `String`

ヒープに確保された、可変・サイズ変更可能な文字列型。

```rust
let mut s = String::new();           // 空のStringを作成
let s = String::from("hello");       // 文字列リテラルから作成
let s = "hello".to_string();         // &str から変換
```

### 内部構造

`String` は内部に3つのフィールドを持つ：

```
String {
    ptr:      ヒープバッファへのポインタ
    len:      現在の文字列の長さ（バイト数）
    capacity: 確保済みのバッファサイズ（バイト数）
}
```

### 文字列の追加・結合

```rust
let mut s = String::from("hello");

// 1文字追加
s.push('!');

// 文字列スライスを追加
s.push_str(" world");

// + 演算子（左辺の所有権を消費する）
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2; // s1 はムーブされ使えなくなる

// format! マクロ（所有権を消費しない）
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = format!("{}{}", s1, s2); // s1, s2 ともに使い続けられる
```

---

## インデックスアクセスの注意点

Rustの `String` は **インデックスによる直接アクセスをサポートしない**。

```rust
let s = String::from("hello");
// let c = s[0]; // コンパイルエラー！
```

理由：UTF-8は可変長エンコーディングのため、バイトインデックスと文字インデックスが一致しない。

```rust
let s = String::from("こんにちは");
// 「こ」は3バイト占める
println!("{}", s.len()); // 15（バイト数）
```

### 正しい文字へのアクセス方法

```rust
let s = String::from("hello");

// バイトスライスで取得（ASCII限定で安全）
let slice = &s[0..3]; // "hel"

// char イテレータで1文字ずつ処理
for c in s.chars() {
    println!("{}", c);
}

// バイト列として処理
for b in s.bytes() {
    println!("{}", b);
}
```

---

## よく使うメソッド

```rust
let s = String::from("  Hello, World!  ");

s.len()                    // バイト数
s.is_empty()               // 空文字列か
s.contains("World")        // 部分文字列の検索
s.starts_with("Hello")     // 前方一致
s.ends_with("!")           // 後方一致
s.to_uppercase()           // 大文字変換（新しいStringを返す）
s.to_lowercase()           // 小文字変換
s.trim()                   // 前後の空白除去（&str を返す）
s.replace("World", "Rust") // 文字列置換
s.split(", ")              // 区切り文字で分割（イテレータを返す）
```

---

## `&str` と `String` の変換

```rust
// &str → String
let s: String = "hello".to_string();
let s: String = String::from("hello");

// String → &str
let owned = String::from("hello");
let slice: &str = &owned;
let slice: &str = owned.as_str();
```

### 関数の引数では `&str` を使うのが慣例

```rust
// 良い例：&str を受け取ることで &str でも String でも渡せる
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

let s = String::from("Alice");
greet(&s);        // String を &str として渡す
greet("Bob");     // &str を直接渡す
```

---

## なぜ `String` と `&str` の2種類があるのか？ ── 言語設計のメリット

他の多くの言語（Java, Python, Go など）では文字列は単一の型で扱われる。
Rustが2種類を持つのは意図的な設計であり、以下のメリットがある。

### 1. 所有権とゼロコストな借用

`String` は所有権を持つため、不要になれば自動でメモリが解放される（GCなし）。
一方 `&str` はそのデータを「借りているだけ」なので、コピーが発生しない。

```rust
fn print_len(s: &str) {
    println!("{}", s.len());
}

let owned = String::from("hello");
print_len(&owned);   // ヒープのデータをコピーせず参照渡し
print_len("world");  // リテラルもそのまま渡せる
```

もし `String` しかなければ、関数に渡すたびにヒープコピーか所有権の移動が必要になる。

### 2. 異なるメモリ領域の文字列を統一的に扱える

`&str` はどこにあるデータでも参照できる抽象型。

| データの場所 | 型 |
|---|---|
| バイナリ内のリテラル | `&'static str` |
| ヒープ上の `String` の一部 | `&str` |
| スタック上のバッファ | `&str` |

文字列の出所を問わず `&str` として受け取れるため、APIが柔軟になる。

### 3. 不変と可変を型で明確に区別できる

`&str` は常に不変。`String` は `mut` 宣言で可変にできる。
これにより、「この関数は文字列を変更しない」という意図をシグネチャで表現できる。

```rust
fn read_only(s: &str) { /* 変更不可 */ }
fn modifiable(s: &mut String) { s.push_str("!"); }
```

コンパイラが不正な変更をコンパイル時に検出できる。

### 4. パフォーマンスの最適化が可能

部分文字列を切り出す際、`String` だとコピーが必要だが、`&str` はポインタ＋長さだけで表現できる。

```rust
let s = String::from("hello world");
let word = &s[0..5]; // "hello" ── コピーなし、O(1)
```

GCを持つ言語では文字列のスライスを安全に実現するためにGCや参照カウントに頼るが、
Rustはライフタイムによってコンパイル時に安全性を保証しつつゼロコストで実現している。

### 他言語との比較

| 言語 | 文字列の扱い | トレードオフ |
|---|---|---|
| Java | `String`（不変）+ `StringBuilder`（可変） | ヒープ割り当て必須、GCに依存 |
| Python | `str`（不変のみ） | 操作のたびにコピーが発生 |
| C++ | `std::string` + `std::string_view` | Rustに近いが安全性はプログラマ任せ |
| Go | `string`（不変） + `[]byte`（可変） | 変換コストが発生、型が異なる |
| **Rust** | `String` + `&str` | ゼロコスト・コンパイル時安全保証 |

Rustの設計は C++ の `std::string` / `std::string_view` に近いが、
ライフタイムによってコンパイラが安全性を強制する点が大きく異なる。

---

## まとめ

- **`&str`**：文字列への借用参照。軽量で関数の引数に向く
- **`String`**：所有権を持つ可変文字列。動的に構築・変更したいときに使う
- 文字列はUTF-8のため、バイトインデックスでの直接アクセスは不可
- `chars()` でUnicode文字単位のイテレーションが可能
- 2種類の型を持つことで **ゼロコスト抽象・安全性・柔軟性** を同時に実現している
