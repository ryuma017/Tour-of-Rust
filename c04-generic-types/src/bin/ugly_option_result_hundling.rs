fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    let v = do_something_that_might_fail(42).unwrap();
    println!("found {}", v);

    let v = do_something_that_might_fail(1).unwrap();
    println!("found {}", v);

    Ok(())
}

// unwrap を使った Option/Result 処理はやっつけな感じ
// match を適切に使ったほうが良い
