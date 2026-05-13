use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let s: String = iter.next().unwrap().parse().unwrap();
    let t: String = iter.next().unwrap().parse().unwrap();

    // ここから問題を解く
}