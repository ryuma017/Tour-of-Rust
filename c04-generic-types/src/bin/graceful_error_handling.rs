fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // 簡潔なエラーハンドリング
    let v = do_something_that_might_fail(42)?; // この演算子`?`が強い

    /* 上と以下のコードは等価
    let v = match do_something_that_might_fail(42) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    */

    println!("found {}", v);
    Ok(())
}
