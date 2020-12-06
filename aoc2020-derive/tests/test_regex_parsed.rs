#[macro_use]
extern crate aoc2020_derive;
#[macro_use]
extern crate lazy_static;

#[test]
fn test_parse_structure() {
    #[regex_parsed(r"foo (\d+) (.+)")]
    #[derive(Debug, Eq, PartialEq)]
    struct Foo {
        i: u32,
        x: String,
    };

    let f: Foo = "foo 234 asdf".parse().unwrap();
    assert_eq!(f, Foo{i: 234, x: "asdf".into()});
}

#[test]
fn test_parse_tuple() {
    #[regex_parsed(r"tup (\d+) (.+)")]
    #[derive(Debug, Eq, PartialEq)]
    struct Bar(u32, String);

    let b: Bar = "tup 123 asdf".parse().unwrap();
    assert_eq!(b, Bar(123, "asdf".into()));
}
