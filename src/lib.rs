use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::error::Error;
use std::str::FromStr;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub fn read_file(p: &str) -> String {
    let mut f = File::open(p).expect("open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("read file");
    s
}

pub fn each_line(p: &str) -> impl Iterator<Item=String> {
    let f = File::open(p).expect("open file");
    let r = BufReader::new(f);
    r.lines().map(|x| x.expect("Error reading line"))
}

pub fn parse_lines<T: FromStr>(p: &str) -> Vec<T>
where <T as FromStr>::Err: std::fmt::Debug {
    let mut vals = Vec::new();
    for l in each_line(p) {
        let n = l.parse::<T>().expect("Parse fail");
        vals.push(n);
    }
    vals
}
