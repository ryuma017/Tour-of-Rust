#![allow(clippy::blacklisted_name)]

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f ドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // do_something(foo);
    // foo.x = 13;
    // 上2つはここではエラー
    // foo は可変に借用されていて、移動や変更ができないから

    f.x = 13;
    // f はここから先では使用されないから、ここでドロップ

    println!("{}", foo.x);

    // 可変な借用はでドロップされているから変更可能
    foo.x = 7;

    // foo の所有権を関数に移動
    do_something(foo);
}
