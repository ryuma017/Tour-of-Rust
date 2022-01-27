// 部分的に定義された構造体
struct BagOfHolding<T> {
    // パラメータ T を渡すことが可能
    item: Option<T>,
}

fn main() {
    // i32 が入るbagに何も入っていない
    // None からは型が決められないため、型指定が必要
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("there's nothing in the bag!")
    } else {
        println!("there's something in the bag!")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("there's something in the bag!")
    } else {
        println!("there's nothing in the bag!")
    }

    match i32_bag.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }
}
