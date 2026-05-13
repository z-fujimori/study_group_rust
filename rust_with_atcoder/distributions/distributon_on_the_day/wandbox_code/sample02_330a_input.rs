use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();

    // vecは可変長の配列です
    let a_vec: Vec<usize> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // ここから問題を解く
}