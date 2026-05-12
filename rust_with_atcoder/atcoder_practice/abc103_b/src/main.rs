use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ss = s.clone() + &s;

    if ss.contains(&t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
