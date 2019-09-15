fn main() {
    let one = String::from("one");
    let two = String::from("two");

    // https://doc.rust-lang.org/std/macro.format.html
    let result = format!("{} {}", one, two);
    println!("result: '{}'", result);
}
