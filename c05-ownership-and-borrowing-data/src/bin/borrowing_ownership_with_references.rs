#![allow(clippy::blacklisted_name)]

struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo; // ここで`&` 演算子を使ってリソースアクセスを借用しないと、
    println!("foo: {}, f: {}", foo.x, f.x); // ここでコンパイラがエラーを吐く
                                            // f ドロップ
                                            // foo ドロップ
}
