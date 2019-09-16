fn main() {
    let one = String::from("one");
    let two = String::from("two");

    // https://doc.rust-lang.org/std/macro.format.html
    let string = format!("{} {}", one, two);
    // let type_check: () = result;
    let result: String = string;
    println!("result: '{}'", result);
}
