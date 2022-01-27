fn example() -> i32 {
    let x = 42;
    // 三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        // 単一の式で値を帰す場合、中括弧は省略できる
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);

    let v = {
        // ブロックのスコープは関数のスコープから分離される
        // 要はブロック外からは参照できんてことか
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    v + 4
}

fn main() {
    println!("from function: {}", example());
}
