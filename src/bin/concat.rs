// Uncomment /* */ blocks to see compiler errors.

#![allow(unused)]

fn main() {
    two_slices();
    two_strings();
    string_and_slice();
    slice_and_string();
}

// The str primitive does not implement Add<&str>
// https://doc.rust-lang.org/std/primitive.str.html#implementations
// Two &str's can not be added using `+`
fn two_slices() {
    let one = String::from("one");
    let two = String::from("two");

    let slice_one = &one;
    let slice_two = &two;

    // Inferred type: deref coercion converts String to &str
    // Compiler error:
    // `+` can not be used to concatenate two `&str` string
    /*
    let three = slice_one + slice_two;
    */

    // Compiler error:
    // `+` can not be used to concatenate two `&str` string
    /*
    let three = String::from(one + two);
    */

    // Explicit type
    let slice_one: &str = &one;
    let slice_two: &str = &two;
    // Compiler error:
    // `+` can not be used to concatenate two `&str` string
    /*
    let three = slice_one + slice_two;
    */
}

// The str primitive does not implement Add<String>
// https://doc.rust-lang.org/std/primitive.str.html#implementations
// Can not add a String to a &str using `+`
fn slice_and_string() {
    let one = "one";
    let two = String::from("two");

    // Compiler error:
    // `+` cannot be used to concatenate a `&str` with a `String`
    /*
    let three = one + two;
    */

    // Compiler error:
    // `+` cannot be used to concatenate a `&str` with a `String`
    /*
    let three = String::from(one + two);
    */
}

// The String type does not implement Add<String>
// https://doc.rust-lang.org/std/string/struct.String.html#implementations
// Can not add two Strings using `+`
fn two_strings() {
    let one = String::from("One");
    let two = String::from("Two");

    // Compiler error:
    // Expected &str, found String
    /*
    let three = one + two;
    */

    // Compiler error:
    // Expected &str, found String
    /*
    let three = String::from(one + two);
    */
}

// The String type does implement Add<&str>
// https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26%27_%20str%3E
// Can add String and &str using `+`
fn string_and_slice() {
    let one = String::from("one");
    let two = "two";
    let three = one + two;

    let one = String::from("one");
    let two = "two";
    let three = String::from(one + two);
}
