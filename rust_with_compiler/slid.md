---
marp: true
theme: default
paginate: true
header: Rust with Compiler
---

# Rust講義: CLIアプリで学ぶ

Cargo単体CLI -> ファイル入出力CLI へステップアップ

- 対象: Rust初学者
- 講義時間: 70分
- 今日のゴール:
  - CargoでCLIを作って実行できる
  - ファイル読み書きができる
  - Option型とResult型を「使いどころ」で説明できる

---

## 今日の進め方（70分）

1. ウォームアップ（5分）
2. Cargo単体CLI: 計算機（20分）
3. Option/Resultをやさしく理解（15分）
4. ファイル入出力CLI: Todo（20分）
5. まとめとミニ演習（10分）

---

## 先に完成イメージ

前半で作るもの: `calc-cli`

```bash
$ cargo run -- add 3 5
8
```

後半で作るもの: `todo-cli`

```bash
$ cargo run -- add "牛乳を買う"
added: 牛乳を買う
```

---

## 初学者向けキーワード

- CLI: 文字で操作するアプリ
- 引数: コマンドの後ろに渡す値
- 型: データの種類（`i32`, `String` など）
- エラー処理: 失敗したときの扱いを決めること

---

## なぜRustでCLI？

- 書いた内容が型で守られる
- エラー処理をサボりにくい
- 実行が速い
- 学習題材としてサイズがちょうどよい

---

## 前半の題材: `calc-cli`

仕様:

```text
calc-cli <op> <a> <b>
op: add | sub | mul | div
```

例:

```bash
cargo run -- add 10 2
cargo run -- mul 4 7
```

---

## 最初の1歩

```bash
cargo new calc-cli
cd calc-cli
cargo run
```

チェック:

- `src/main.rs` が実行開始地点
- 引数を渡すときは `cargo run -- ...`

---

## 引数を見てみる

```rust
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("args = {:?}", args);
}
```

```bash
cargo run -- add 3 5
```

---

## ステップ1: まずは動く版

```rust
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 4 {
		eprintln!("usage: calc-cli <add|sub|mul|div> <a> <b>");
		return;
	}

	let op = &args[1];
	let a: i32 = args[2].parse().unwrap();
	let b: i32 = args[3].parse().unwrap();

	let ans = match op.as_str() {
		"add" => a + b,
		"sub" => a - b,
		"mul" => a * b,
		"div" => a / b,
		_ => {
			eprintln!("unknown op: {}", op);
			return;
		}
	};

	println!("{}", ans);
}
```

---

## ステップ1の課題

- `parse().unwrap()` は失敗時に終了（panic）
- `div` は0除算の可能性
- エラー理由を整理して返せない

ここから Option型 / Result型 を使って改善する。

---

## Option型を直感で

「値がある or ない」を安全に表す箱。

```rust
enum Option<T> {
	Some(T),
	None,
}
```

たとえば:

- 配列の先頭を取る（空なら先頭がない）
- 検索結果（見つからないことがある）

---

## Option型の読み方

```rust
let tasks = vec!["Rust".to_string(), "CLI".to_string()];

match tasks.first() {
	Some(t) => println!("先頭: {}", t),
	None => println!("空です"),
}
```

ポイント:

- `Some(...)` は値あり
- `None` は値なし

---

## Result型を直感で

「成功した値」か「失敗の情報」かを表す箱。

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

たとえば:

- 文字列を数値へ変換
- ファイルを開く

---

## `?` は何をしている？

`?` の意味:

- `Ok(v)` なら中身 `v` を取り出して続行
- `Err(e)` ならその場で関数を終了して返す

```rust
fn parse_num(s: &str) -> Result<i32, std::num::ParseIntError> {
	let n = s.parse::<i32>()?;
	Ok(n)
}
```

---

## ステップ2: `calc-cli` を安全化

```rust
use std::env;
use std::error::Error;

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {}", e);
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 4 {
		return Err("usage: calc-cli <add|sub|mul|div> <a> <b>".into());
	}

	let op = &args[1];
	let a: i32 = args[2].parse()?;
	let b: i32 = args[3].parse()?;

	let ans = match op.as_str() {
		"add" => a + b,
		"sub" => a - b,
		"mul" => a * b,
		"div" => {
			if b == 0 {
				return Err("division by zero".into());
			}
			a / b
		}
		_ => return Err(format!("unknown op: {}", op).into()),
	};

	println!("{}", ans);
	Ok(())
}
```

---

## ここまでの理解チェック

1. 値がない可能性: Option型
2. 失敗する可能性: Result型
3. エラー伝播を短く書く: `?`

---

## 後半の題材: `todo-cli`

仕様（最小）:

```text
todo-cli add <task>
todo-cli list
```

保存先: 実行ディレクトリの `todo.txt`

---

## 使う標準ライブラリ

- `std::fs` : ファイル読み書き
- `std::io::Write` : ファイルへ追記
- `std::path::Path` : ファイルの存在確認

---

## `todo-cli` 実装（読みやすい版）

```rust
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const FILE_PATH: &str = "todo.txt";

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {}", e);
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		return Err("usage: todo-cli <add|list> [task]".into());
	}

	match args[1].as_str() {
		"add" => {
			if args.len() < 3 {
				return Err("usage: todo-cli add <task>".into());
			}
			let task = args[2..].join(" ");
			add_task(&task)?;
			println!("added: {}", task);
		}
		"list" => {
			let tasks = read_tasks()?;
			if tasks.is_empty() {
				println!("(no tasks)");
			} else {
				for (i, task) in tasks.iter().enumerate() {
					println!("{}. {}", i + 1, task);
				}
			}
		}
		_ => return Err("unknown command. use add or list".into()),
	}

	Ok(())
}

fn add_task(task: &str) -> Result<(), Box<dyn Error>> {
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(FILE_PATH)?;

	writeln!(file, "{}", task)?;
	Ok(())
}

fn read_tasks() -> Result<Vec<String>, Box<dyn Error>> {
	if !Path::new(FILE_PATH).exists() {
		return Ok(vec![]);
	}

	let content = fs::read_to_string(FILE_PATH)?;
	let tasks = content
		.lines()
		.map(|line| line.trim())
		.filter(|line| !line.is_empty())
		.map(String::from)
		.collect();

	Ok(tasks)
}
```

---

## このコードでの Option型

「先頭の1件だけ見たい」なら:

```rust
let tasks = read_tasks()?;

match tasks.first() {
	Some(t) => println!("first task: {}", t),
	None => println!("task is empty"),
}
```

---

## このコードでの Result型

`?` が付く処理は「失敗するかもしれない処理」。

- `open(FILE_PATH)?`
- `writeln!(...)?`
- `read_to_string(FILE_PATH)?`

失敗時は `run()` まで戻して、`main`で表示。

---

## 初学者がつまずきやすい点

1. `cargo run --` の `--` を忘れる
2. `parse()` の戻り値がResult型だと気づかない
3. `unwrap()` を多用してしまう
4. `String` と `&str` の違いで混乱する

---

## 実装の型を意識するコツ

- 迷ったら戻り値の型を見る
- エラーが起こる操作には `Result`
- 値がない可能性には `Option`

覚え方:

- Option = ある/ない
- Result = 成功/失敗

---

## 70分進行案（授業運用しやすい版）

- 0-5分: 完成物デモ
- 5-15分: 引数と`match`
- 15-25分: `calc-cli` 実装
- 25-40分: Option/Result/`?`
- 40-55分: `todo-cli` 実装
- 55-65分: 受講者ハンズオン
- 65-70分: まとめと課題

---

## ハンズオン課題（やさしめ）

1. `calc-cli` に `mod` を追加
2. `todo-cli` に `--help` を追加
3. エラーメッセージを日本語化

---

## ハンズオン課題（発展）

1. `todo-cli done <番号>` を実装
2. 完了済みタスクを別表示
3. JSON保存へ変更（`serde_json`）

---

## まとめ

- 小さなCLIはRust学習に最適
- Option型は「値がない」を安全に扱える
- Result型は「失敗」を安全に扱える
- `?` でエラー処理が読みやすくなる

次の一歩:

- まず動くものを作る
- 次に型とエラー処理を整える
- 最後に機能を増やす



