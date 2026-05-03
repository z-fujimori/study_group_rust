use proconio::input;

fn main() {
    input! {
        k: usize,
        s: usize,
    }

    let mut count = 0;

    for x in 0..=k {
        for y in 0..=k {
            let z = s - x - y;
            if 0 <= z && z <= k {
                count += 1;

            }
        }
    }

    println!("{}", count);
}
