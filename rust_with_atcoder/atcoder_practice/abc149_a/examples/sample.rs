use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ans = t + &s;

    println!("{}", ans);
    // println!("{}", s);
    // println!("{}", t);
}
