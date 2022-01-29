#![allow(clippy::blacklisted_name)]

struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    // f への可変な参照はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    // do_something で可変な参照はドロップされるから
    // 別の参照を作ることが可能
    do_something(&mut foo);
    // foo ドロップ
}
