#[macro_use]
extern crate aoc2020_derive;
#[macro_use]
extern crate lazy_static;
extern crate regex;

#[regex_parsed(r"(.) \w+ (\w+)")]
#[derive(Debug)]
struct Test {
    x: f32,
    xyz: String,
}

#[regex_parsed(r"asdf (.)")]
#[derive(Debug)]
struct Test2 {
    c: char,
}

fn main() {
    let foo: Test = "1 asdfasdf ccccc".parse().unwrap();
    let foo2: Test2 = "asdf t".parse().unwrap();
    dbg!(foo);
    dbg!(foo2);
}
