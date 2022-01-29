struct Foo {
    x: i32,
}

// 関数の引数や戻り値のライフタイムをコンパイラが解決できない場合に、明示的に指定する
// foo_b と戻り値はライフタイムを共有
// foo_a のライフタイムは別
fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    &foo_b.x
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    // ここから先は foo_b のライフタイムしか存在しないため、
    // foo_a はここでドロップ
    println!("{}", x);
    // x ドロップ
    // foo_b ドロップ
}
