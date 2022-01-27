#![allow(clippy::approx_constant)]

// 部分的に定義された構造体
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    // generic型を使用すると、型はコンパイル時に作成される。
    // ::<> (turbofish) で明示的に型を指定
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };

    // generic型でも型推論可能
    let float_bag = BagOfHolding { item: 3.14 };
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );
}
