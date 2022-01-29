#![allow(clippy::approx_constant, clippy::redundant_static_lifetimes)]

static PI: f64 = 3.1415;

fn main() {
    // スタティック変数は関数スコープでも定義可能
    static mut SECRET: &'static str = "swordfish";

    // 文字列リテラルは 'static ライフタイム
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // ルールを破ることはできるが、それを明示する必要がある
    unsafe {
        // 文字列リテラルは 'static だから SECRET に代入可能
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}
