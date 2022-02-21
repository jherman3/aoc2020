use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub fn read_file(p: &str) -> String {
    let mut f = File::open(p).expect("open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("read file");
    s
}

pub fn each_line(p: &str) -> impl Iterator<Item = String> {
    let f = File::open(p).expect("open file");
    let r = BufReader::new(f);
    r.lines()
        .map(|x| x.expect("Error reading line").trim().to_owned())
}

pub fn parse_lines<T: FromStr>(p: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut vals = Vec::new();
    for l in each_line(p) {
        let n = l.parse::<T>().expect("Parse fail");
        vals.push(n);
    }
    vals
}

pub struct Neighbors {
    i: i32,
    j: i32,
    m: i32,
    n: i32,
    idx: usize,
}

impl Neighbors {
    pub fn new(i: usize, j: usize, m: usize, n: usize) -> Self {
        Self {
            i: i as i32,
            j: j as i32,
            m: m as i32,
            n: n as i32,
            idx: 0,
        }
    }
}

impl Iterator for Neighbors {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > 8 {
            return None;
        }
        let dx = (self.idx / 3) as i32 - 1;
        let dy = (self.idx % 3) as i32 - 1;
        self.idx += 1;
        if dx == 0 && dy == 0 {
            return self.next();
        }
        let x = self.i + dx;
        let y = self.j + dy;
        if x >= 0 && y >= 0 && x < self.m && y < self.n {
            Some((x as usize, y as usize))
        } else {
            self.next()
        }
    }
}
