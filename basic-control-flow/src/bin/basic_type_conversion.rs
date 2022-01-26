fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    let f = false;
    println!("true -> {}, false -> {}", t as u8, f as u8); //true -> 1, false -> 0
}