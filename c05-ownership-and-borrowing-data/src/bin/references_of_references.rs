#![allow(clippy::blacklisted_name)]

struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    &a.x
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13; // 参照元の foo.x の値を 13 に設定
             // x はここでドロップされるから、不変な参照が作成可能
    let y = do_something(&foo);
    println!("{}", y);
    // y ドロップ
    // foo ドロップ
}
