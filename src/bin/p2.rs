#[macro_use]
extern crate aoc2020_derive;
#[macro_use]
extern crate lazy_static;

use aoc2020::parse_lines;

#[regex_parsed(r"(\d+)-(\d+) (.): (.+)")]
struct Entry {
    begin: usize,
    end: usize,
    c: char,
    pass: String,
}

fn main() {
    let entries: Vec<Entry> = parse_lines("inputs/p2.txt");
    let mut valid_p1 = 0;
    let mut valid_p2 = 0;
    for e in entries {
        let cnt = e.pass.chars().filter(|&x| x == e.c).count();
        if cnt >= e.begin && cnt <= e.end {
            valid_p1 += 1;
        }
        let b = e.pass.as_bytes();
        if (b[e.begin - 1] == e.c as u8) ^ (b[e.end - 1] == e.c as u8) {
            valid_p2 += 1;
        }
    }
    dbg!(valid_p1, valid_p2);
}
