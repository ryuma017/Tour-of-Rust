struct Location(i32, i32);

fn main() {
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}