use proconio::input;

fn main() {
    input! {
        k: i32,
    }

    let mut ans_sum = 0;

    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans_sum += gcd(gcd(a, b), c);
            }
        }
    }

    println!("{}", ans_sum);
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
