fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// main を値を返さないが、エラーは返すことがある
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(13);

    match result {
        Ok(v) => println!("found {}", v),
        Err(_e) => {
            // エラーハンドリング

            // エラーを main から返す
            return Err(String::from("something went wrong in main!"));
        }
    }

    // Result の Ok の中にある unit 値によって、
    // すべてが正常であることを表現
    Ok(())
}
