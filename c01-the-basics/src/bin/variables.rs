fn main() {
    // xの型を推論
    let x = 13;
    println!("{}", x);

    // 型を明示的に指定することもできる
    let x: f64 = 1.23456;
    println!("{}", x);

    // 宣言の後で初期化もできるけどあんま使わんほうが良い
    let x;
    x = 0;
    println!("{}", x);
}
