#![allow(clippy::blacklisted_name)]

struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // 最初に foo がドロップ
    // 次に foo.bar がドロップ
}
