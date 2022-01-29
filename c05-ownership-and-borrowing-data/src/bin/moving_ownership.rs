#![allow(clippy::blacklisted_name)]

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f ドロップ
}

fn main() {
    let foo = Foo { x: 42 };
    // foo の所有権が do_something にムーブ
    do_something(foo);
    // foo は使えなくなる
}
