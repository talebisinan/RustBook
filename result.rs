fn might_fail(num: i32) -> Result<i32, String> {
    if num == 42 {
        Ok(42)
    } else {
        Err(String::from("It's not the answer"))
    }
}

fn main() -> Result<(), String> {
    let result = might_fail(33);
    match result {
        Ok(r) => println!("found {}", r),
        Err(_e) => return Err(String::from("Error occured")),
    }
    Ok(())
}
