#![allow(unused)]

// Uncomment /* */ blocks within functions to see compiler errors.

/*
Links to documentation

str, the string primitive:
https://doc.rust-lang.org/std/primitive.str.html

str, trait implementations:
https://doc.rust-lang.org/std/primitive.str.html#implementations

String, from std::string::String:
https://doc.rust-lang.org/std/string/struct.String.html

String, the Add<&str> trait:
https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26%27_%20str%3E

String, the Deref trait:
https://doc.rust-lang.org/std/string/struct.String.html#impl-Deref
*/

fn main() {
    slice_and_slice();
    slice_and_string();
    slice_and_string_borrow();

    // Slices are already borrowed

    string_and_slice();
    string_and_string();
    string_and_string_borrow();

    string_borrow_and_slice();
    string_borrow_and_string();
    string_borrow_and_string_borrow();
}

// The str primitive does not implement Add<&str>
// Can not add &str and &str using `+`
fn slice_and_slice() {
    let one = String::from("one");
    let two = String::from("two");

    let slice_one: &str = &one;
    let slice_two: &str = &two;

    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = slice_one + slice_two;
    */

    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = String::from(slice_one + slice_two);
    */
}

// The str primitive does not implement Add<String>
// Can not add &str and String using `+`
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

// The str primitive does not implement Add<&String>
// String does implement Deref, so &String is coerced into &str
// Can not add &str and &String using `+` because
// can not add &str and &str
fn slice_and_string_borrow() {
    let one = String::from("one");
    let two = String::from("two");

    let slice_one: &str = &one;
    let borrow_two: &String = &two;

    // Rust coerces &String into &str using the Deref trait
    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = slice_one + borrow_two;
    */
}

// The String type *does* implement Add<&str>
// Can add String and &str using `+`
fn string_and_slice() {
    let one = String::from("one");
    let two = String::from("two");

    let slice_two: &str = &two;

    // Success:
    // "onetwo"
    let three = one + slice_two;
}

// The String type does not implement Add<String>
// Can not add String and String using `+`
fn string_and_string() {
    let one = String::from("One");
    let two = String::from("Two");

    // Compiler error:
    // expected &str, found struct `std::string::String`
    // Rust does allow String + &str, so it suggests that the second argument be a &str
    /*
    let three = one + two;
    */

    // Compiler error:
    // expected &str, found struct `std::string::String`
    /*
    let three = String::from(one + two);
    */
}

// The String type does not implement Add<&String>
// String does implement Deref, so &String is coerced into &str
// Can add String and &String using `+` because
// can add String and &str
fn string_and_string_borrow() {
    let one = String::from("one");
    let two = String::from("two");

    let borrow_two: &String = &two;

    // Rust coerces &String into &str using the Deref trait
    // Success:
    // "onetwo"
    let three = one + borrow_two;
}

// The String type does implement Add<&str>
// String does implement Deref, so &String is coerced into &str
// Can not add &String and &str using `+` because
// can not add &str and &str
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

// The String type does not implement Add<String>
// String does implement Deref, so &String is coerced into &str
// Can not add &String and String using `+` because
// can not add &str and String
fn string_borrow_and_string() {
    let one = String::from("one");
    let two = String::from("two");

    let borrow_one: &String = &one;

    // Compiler error:
    // `+` cannot be used to concatenate a `&str` with a `String
    /*
    let three = borrow_one + two;
    */
}

// String does implement Deref, so &String is coerced into &str
// Can not add &String and &String using `+` because
// can not add &str and &str
fn string_borrow_and_string_borrow() {
    let one = String::from("one");
    let two = String::from("two");

    let borrow_one: &String = &one;
    let borrow_two: &String = &two;

    // Rust coerces &String into &str using the Deref trait
    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = borrow_one + borrow_two;
    */

    // Inferred type: variable binds as &String, coerced into &str by the `+` operator
    let borrow_one = &one;
    let borrow_two = &two;

    // Compiler error:
    // `+` cannot be used to concatenate two `&str` strings
    /*
    let three = borrow_one + borrow_two;
    */
}
