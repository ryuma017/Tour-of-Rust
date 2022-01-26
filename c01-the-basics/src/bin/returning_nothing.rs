fn make_nothing() -> () {
    return ();
}

fn make_nothing2() {
    // 戻り値が指定されていないと空のタプル(unit)を返す。
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}