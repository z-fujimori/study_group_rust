fn main() {
    println!("Hello, world!");
}
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    // ここから問題を解く
    let ss = s.clone() + &s;

    if ss.contains(&t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
