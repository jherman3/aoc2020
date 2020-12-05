#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::error::Error;
use std::str::FromStr;

use regex::Regex;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub fn read_file(p: &str) -> String {
    let mut f = File::open(p).expect("open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("read file");
    s
}

pub fn line_input<T: FromStr>(p: &str) -> Vec<T>
where <T as FromStr>::Err: std::fmt::Debug {
    let f = File::open(p).expect("open file");
    let r = BufReader::new(f);
    let mut vals = Vec::new();
    for l in r.lines() {
        let n = l.expect("No capture").parse::<T>().expect("Parse fail");
        vals.push(n);
    }
    vals
}

macro_rules! regex_tuple {
    ($f:ident, $r:expr, $($ts:ty),*) => {
        mod $f {
            lazy_static! {
                static ref REG: crate::Regex = crate::Regex::new($r).unwrap();
            }
            pub fn parse(s: &str) -> ($($ts,)*) {
                let parsed = REG.captures(s).unwrap();
                let mut i = 0;
                ($(
                    parsed.get({i += 1; i}).unwrap().as_str().parse::<$ts>().unwrap(),
                )*)
            }
        }
    };
}

regex_tuple!(parse_test, r"(\d+)-(\d+) (.): (.+)", u32, u32, char, String);
regex_tuple!(
    parse_test2,
    r"(\d+)-(\d+) (.): (.+)",
    u32,
    u32,
    char,
    String
);

pub fn foo() {
    let z = parse_test::parse("2-34 d: asdfasdfd");
    let z1 = parse_test2::parse("2-34 d: asdfasdfd");
    dbg!(z, z1);
}
