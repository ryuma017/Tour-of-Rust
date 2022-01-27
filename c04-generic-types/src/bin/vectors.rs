#![allow(clippy::vec_init_then_push)]

fn main() {
    // 型を明示的に指定
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    for num in i32_vec.iter() {
        println!("{}", num);
    }

    // 型を自動的に推論
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    for num in float_vec.iter() {
        println!("{}", num)
    }

    // マクロ 綺麗に簡単に書ける
    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}
