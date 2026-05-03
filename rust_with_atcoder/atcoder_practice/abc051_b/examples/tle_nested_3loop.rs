use proconio::input;

fn main() {
    input! {
        k: usize,
        s: usize,
    }

    let mut count = 0;

    for x in 0..=k {
        for y in 0..=k {
            for z in 0..=k {
                if x + y + z == s {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
