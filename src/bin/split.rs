// https://doc.rust-lang.org/std/string/struct.String.html#method.split
// https://doc.rust-lang.org/std/str/struct.Split.html
/*
pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> where
    P: Pattern<'a>,

pub struct Split<'a, P>(_)
 where
    P: Pattern<'a>;
*/

fn main() {
    let string = String::from("a|b");
    let split: core::str::Split<char> = string.split('|');
    dbg!(split);

    // A Split implements the Iterator trait - it's an iterator!
    let split = string.split('|');
    for item in split {
        dbg!(item);
    }

    // To split on newlines, use lines()
    // https://doc.rust-lang.org/std/string/struct.String.html#method.lines
    // https://doc.rust-lang.org/std/str/struct.Lines.html
    // pub struct Lines<'a>(_);
    let string = String::from("a\nb");
    let lines = string.lines();
    // This also returns an iterator
    for line in lines {
        dbg!(line);
    }
}
