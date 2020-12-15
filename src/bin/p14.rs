#![feature(str_split_once)]

use aoc2020::each_line;
use std::collections::HashMap;

fn p1() {
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = u64::MAX;
    let mut mem = HashMap::new();
    for l in each_line("inputs/p14.txt") {
        if l.starts_with("mask = ") {
            let (_, mask) = l.split_once(" = ").unwrap();
            or_mask = 0;
            and_mask = u64::MAX;
            for (idx, c) in mask.chars().rev().enumerate() {
                match c {
                    '0' => {
                        and_mask &= !(1 << idx);
                    }
                    '1' => {
                        or_mask |= 1 << idx;
                    }
                    _ => {}
                }
            }
        } else {
            // Assume format mem[x] = y
            let (_, rest) = l.split_once("[").unwrap();
            let (x, y) = rest.split_once("] = ").unwrap();
            let addr: u64 = x.parse().unwrap();
            let val: u64 = y.parse().unwrap();
            mem.insert(addr, val & and_mask | or_mask);
        }
    }
    let p1: u64 = mem.values().sum();
    dbg!(p1);
}

fn p2() {
    let mut mem = HashMap::new();
    let mut mask = String::new();
    for l in each_line("inputs/p14.txt") {
        if l.starts_with("mask = ") {
            let (_, new) = l.split_once(" = ").unwrap();
            mask = new.to_owned();
        } else {
            // Assume format mem[x] = y
            let (_, rest) = l.split_once("[").unwrap();
            let (x, y) = rest.split_once("] = ").unwrap();
            let addr: u64 = x.parse().unwrap();
            let val: u64 = y.parse().unwrap();
            for a in addrs(&mask, addr) {
                mem.insert(a, val);
            }
        }
    }
    let p2: u64 = mem.values().sum();
    dbg!(p2);
}

fn addrs(mask: &str, addr: u64) -> Vec<u64> {
    let mut res = Vec::new();

    fn helper(m: &[u8], cur: u64, v: &mut Vec<u64>) {
        if m.len() == 0 {
            v.push(cur);
            return;
        }
        let n = m.len() - 1;
        if m[0] == '0' as u8 {
            helper(&m[1..], cur, v);
        } else if m[0] == '1' as u8 {
            helper(&m[1..], cur | (1 << n), v);
        } else {
            helper(&m[1..], cur & !(1 << n), v);
            helper(&m[1..], cur | (1 << n), v);
        }
    }
    helper(mask.as_bytes(), addr, &mut res);
    res
}

fn main() {
    p1();
    p2();
}
