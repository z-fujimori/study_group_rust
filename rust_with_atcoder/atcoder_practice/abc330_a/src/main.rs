use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
        list_a: [i32; n],
    }

    let mut count = 0;

    for a in &list_a {
        if a >= l {
            count += 1;
        }
    }

    println!("{}", count);
}
