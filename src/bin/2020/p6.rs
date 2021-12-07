use aoc2020::read_file;

use std::collections::HashSet;

fn main() {
    let s = read_file("inputs/p6.txt");
    let ls: Vec<_> = s.split("\n\n").collect();
    let anyone: usize = ls.iter().map(count_unique).sum();
    dbg!(anyone);
    let everyone: usize = ls.iter().map(count_everyone).sum();
    dbg!(everyone);
}

fn count_unique<S: AsRef<str>>(s: S) -> usize {
    let s: HashSet<char> = s.as_ref().chars().filter(|c| c.is_alphabetic()).collect();
    s.len()
}

fn count_everyone<S: AsRef<str>>(s: S) -> usize {
    let mut all: HashSet<char> = ('a'..='z').collect();
    for l in s.as_ref().lines() {
        let x: HashSet<char> = l.chars().collect();
        all = all.intersection(&x).copied().collect();
    }
    all.len()
}
