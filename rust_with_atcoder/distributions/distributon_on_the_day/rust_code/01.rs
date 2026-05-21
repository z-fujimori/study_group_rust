use proconio::input;
fn main() {
    input! {
        n: usize,
        a_list: [i32; n],
    }

    for a in a_list {
        println!("{}", a);
    }
}
