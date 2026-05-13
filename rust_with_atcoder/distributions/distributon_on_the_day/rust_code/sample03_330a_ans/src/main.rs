use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
        a_list: [i32; n],
    }
    
    // ここから問題を解く
    let mut count = 0;

    for a in a_list {
        if a >= l {
            count += 1;
        }
    }

    println!("{}", count);
}
