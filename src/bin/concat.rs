// Uncomment /* */ blocks to see compiler errors.

#![allow(unused)]

fn main() {
    two_slices();
    two_strings();
    slice_and_string();
    slice_and_string_borrow();
    string_and_slice();
    string_borrow_and_slice();
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
    let one = String::from("one");
    let two = String::from("two");

    let slice_one: &str = &one;

    // Compiler error:
    // `+` cannot be used to concatenate a `&str` with a `String`
    /*
    let three = slice_one + two;
    */

    // Compiler error:
    // `+` cannot be used to concatenate a `&str` with a `String`
    /*
    let three = String::from(slice_one + two);
    */
}

fn slice_and_string_borrow() {
    let one = String::from("one");
    let two = String::from("two");

    let slice_one: &str = &one;
    let borrow_two: &String = &two;

    // The &String is converted to a &str
    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = slice_one + borrow_two;
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
    let two = String::from("two");
    let slice_two: &str = &two;
    let three = one + slice_two;

    let one = String::from("one");
    let two = String::from("two");
    let borrow_two: &str = &two;
    let three = String::from(one + borrow_two);
}

fn string_borrow_and_slice() {
    let one = String::from("one");
    let two = String::from("two");
    let borrow_one: &String = &one;
    let slice_two: &str = &two;

    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = borrow_one + slice_two;
    */
}
