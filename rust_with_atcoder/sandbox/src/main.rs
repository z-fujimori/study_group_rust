fn main() {
    let s = String::from("Hello, world!"); // "Hello, world!".to_string()でもOK
    println!("{}", s);

    let s2 = s; // sの所有権がs2に移動する
    println!("{}", s2);
    // println!("{}", s); // エラー: sはs2に所有権が移動しているため、sは使用できない
}
