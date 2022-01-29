#![allow(clippy::blacklisted_name)]

fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // 所有者の値を取得
    *f = 13; // 参照の所有者の値を設定
    println!("{}", bar);
    println!("{}", foo);
}

// 42
// 13
