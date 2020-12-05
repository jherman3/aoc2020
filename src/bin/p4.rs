#![feature(str_split_once)]

#[macro_use]
extern crate aoc2020_derive;
#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

use aoc2020::read_file;

use std::collections::{HashMap, HashSet};

const REQ: &[&'static str] = &[
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /* "cid" */
];

fn p1() {
    let s = read_file("inputs/p4.txt");
    let r: HashSet<&'static str> = REQ.iter().map(|x| *x).collect();
    let mut valid = 0;
    for pass in s.split("\n\n") {
        let mut fields = HashSet::new();
        for kv in pass.split_whitespace() {
            let (k, _) = kv.split_once(":").expect("no :");
            fields.insert(k);
        }
        if r.difference(&fields).count() == 0 {
            valid += 1;
        }
    }
    dbg!(valid);
}

fn is_valid<F: Fn(&str) -> bool>(fields: &HashMap<&str, &str>, f: &str, validator: F) -> bool {
    if let Some(s) = fields.get(f) {
        validator(*s)
    } else {
        false
    }
}

fn valid_byr(byr: &str) -> bool {
    if let Ok(i) = byr.parse::<u32>() {
        (1920..=2002).contains(&i)
    } else {
        false
    }
}

fn valid_iyr(iyr: &str) -> bool {
    if let Ok(i) = iyr.parse::<u32>() {
        (2010..=2020).contains(&i)
    } else {
        false
    }
}

fn valid_eyr(eyr: &str) -> bool {
    if let Ok(i) = eyr.parse::<u32>() {
        (2020..=2030).contains(&i)
    } else {
        false
    }
}

#[regex_parsed(r"(\d+)(cm|in)")]
struct Height {
    num: u32,
    unit: String,
}

fn valid_hgt(hgt: &str) -> bool {
    if let Ok(h) = hgt.parse::<Height>() {
        if h.unit == "cm" {
            (150..=193).contains(&h.num)
        } else if h.unit == "in" {
            (59..=76).contains(&h.num)
        } else {
            false
        }
    } else {
        false
    }
}

fn valid_ecl(ecl: &str) -> bool {
    let valids = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valids.contains(&ecl)
}

fn valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<u64>().is_ok()
}

lazy_static! {
    static ref HCL_REG: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
}

fn valid_hcl(hcl: &str) -> bool {
    HCL_REG.is_match(hcl)
}

fn p2() {
    let s = read_file("inputs/p4.txt");
    let mut valid = 0;
    for pass in s.split("\n\n") {
        let mut fields = HashMap::new();
        for kv in pass.split_whitespace() {
            let (k, v) = kv.split_once(":").expect("no :");
            fields.insert(k, v);
        }
        if is_valid(&fields, "byr", valid_byr)
            && is_valid(&fields, "iyr", valid_iyr)
            && is_valid(&fields, "eyr", valid_eyr)
            && is_valid(&fields, "hgt", valid_hgt)
            && is_valid(&fields, "hcl", valid_hcl)
            && is_valid(&fields, "ecl", valid_ecl)
            && is_valid(&fields, "pid", valid_pid)
        {
            valid += 1;
        }
    }
    dbg!(valid);
}

fn main() {
    p1();
    p2();
}
