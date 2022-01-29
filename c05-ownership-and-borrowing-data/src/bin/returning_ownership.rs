#![allow(dead_code, unused_variables, clippy::blacklisted_name)]

struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // 所有権は外に移動
}

fn main() {
    let foo = do_something();
    // foo が所有者になる
    // 関数のスコープの終端により、foo はドロップ
}
