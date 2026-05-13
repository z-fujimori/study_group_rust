use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    // ここから問題を解く
    let ans = t + &s;

    println!("{}", ans);
    // println!("{}", s);
    // println!("{}", t);
}
