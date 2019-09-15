# Strings

Hey, don't forget this.


## The Skinny

There are two types of strings:

  * `str` primitive type ("string slice")
  * `String` struct

`str` from string literals vs `str` as borrowed from a `String`.

Strings implement the [`Deref` trait](https://doc.rust-lang.org/std/string/struct.String.html#deref)


## TODO

  * Think more about slices and `&[..]` vs `&`


## Links

Good:

  * [so](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933)

To read:

  * [str vs String](http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html#fn:1)
  * [reasons](http://dnsh.io/music/2016/10/06/string-concatenation-in-rust-is-not-tivial/)
  * [RFC on coercions](https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md)
  * [idioms](https://github.com/rust-unofficial/patterns/blob/master/idioms/concat-format.md)
  * [users 1](https://users.rust-lang.org/t/joining-str-many-ways/7790/8)
  * [best way](https://users.rust-lang.org/t/best-way-to-do-string-concatenation-in-2019-status-quo/24004)
  * [right way](https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/5)
