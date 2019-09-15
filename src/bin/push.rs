fn main() {
    push_str_primitive();
    push_string_type();
}

// String implements push_str(&str)
// https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
fn push_str_primitive() {
    let mut one = String::from("one");
    one.push_str("two");
    println!("result: {:?}", one);
}

// String does not implement push_str(&String) but it does use deref corecion to get the inner &str
fn push_string_type() {
    let mut one = String::from("one");
    let two = String::from("two");

    one.push_str(&two);
    println!("result: {:?}", one);
}
