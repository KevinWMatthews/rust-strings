#![allow(unused)]

// References:
// https://doc.rust-lang.org/std/string/struct.String.html
// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str/23977218#23977218

fn main() {
    ampersand();
    slices();
    method_as_str();
    method_as_ref();
}

fn ampersand() {
    let one = String::from("one");

    // By default, `&` borrows a &String
    let borrow = &one;
    // let type_check: &() = borrow;

    // Explicitly borrow the str within a String
    let borrow = &*one;
    // let type_check: &() = borrow;

    // Strings implement the Deref trait, so borrows are coerced into a &str
    // using deref coercion
    let borrow: &str = &one;
    // let type_check: &() = borrow;
}

fn method_as_str() {
    let one = String::from("one");

    // Alternatively, borrow the String's str using the as_str() method
    // https://doc.rust-lang.org/std/string/struct.String.html#method.as_str
    // As its name suggests, this returns a &str.
    let borrow = one.as_str();
    // let type_check: &() = borrow;
}

fn method_as_ref() {
    let one = String::from("one");

    // String implements AsRef for multiple types
    // https://doc.rust-lang.org/std/string/struct.String.html#implementations
    // Rust *can not* infer the type!
    //
    // Compiler error:
    // type annotations required: cannot resolve `std::string::String: std::convert::AsRef<_>`
    /*
    let borrow = one.as_ref();
    */

    // String implements AsRef<str>
    // https://doc.rust-lang.org/std/string/struct.String.html#impl-AsRef%3Cstr%3E
    let borrow: &str = one.as_ref();
    // let type_check: &() = borrow;

    // String does not implement AsRef<String>
    //
    // Compiler error:
    // the trait `std::convert::AsRef<std::string::String>` is not implemented for `std::string::String`
    /*
    let borrow: &String = one.as_ref();
    */
}

fn slices() {
    // https://doc.rust-lang.org/std/primitive.slice.html
    // https://doc.rust-lang.org/std/slice/index.html
    let one = String::from("one");

    // This returns the entire str within the String
    let borrow = &one[..];
    // let type_check: &() = borrow;
}
