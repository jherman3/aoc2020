#![feature(vec_remove_item)]

use aoc2020::read_file;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate aoc2020_derive;

#[regex_parsed(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)")]
struct Field {
    name: String,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

fn main() {
    let f = read_file("inputs/p16.txt");
    let mut parts = f.split("\n\n").into_iter();
    let departures = parts.next().unwrap();
    let my_ticket = parts.next().unwrap();
    let nearby_tickets = parts.next().unwrap();

    let mut fields = Vec::new();
    for l in departures.lines() {
        let field: Field = l.parse().unwrap();
        fields.push(field);
    }
    let mut invalid_sum = 0;
    let mut valid_tickets = Vec::new();
    for t in nearby_tickets.lines().skip(1) {
        let fs: Vec<_> = t.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        let mut ticket_valid = true;
        for i in &fs {
            if !valid(*i, &fields) {
                invalid_sum += i;
                ticket_valid = false;
            }
        }
        if ticket_valid {
            valid_tickets.push(fs);
        }
    }
    dbg!(invalid_sum);

    // field name -> index in tickets
    let m = valid_tickets[0].len();
    let mut poss: HashMap<&str, HashSet<usize>> = HashMap::new();
    for f in &fields {
        poss.insert(&f.name, (0..m).collect());
    }
    let mut transposed = Vec::new();
    for i in 0..m {
        let mut r = Vec::new();
        for j in 0..valid_tickets.len() {
            r.push(valid_tickets[j][i]);
        }
        transposed.push(r);
    }

    let mut updated = true;
    while updated {
        updated = false;
        for f in &fields {
            for (i, v) in transposed.iter().enumerate() {
                // eliminate possibility
                if !v.iter().all(|&x| check_field(x, f)) {
                    poss.get_mut(f.name.as_str()).unwrap().remove(&i);
                }
            }
        }
        let mut confirmed = Vec::new();
        for indexes in poss.values() {
            if indexes.len() == 1 {
                confirmed.push(*indexes.iter().next().unwrap());
            }
        }
        for c in confirmed {
            for x in poss.values_mut() {
                if x.len() > 1 {
                    updated = true;
                    x.remove(&c);
                }
            }
        }
    }
    let mut total: u64 = 1;
    let my_ticket: Vec<_> = my_ticket
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    for (field, indexes) in poss {
        if field.starts_with("departure") {
            let idx = *indexes.iter().next().unwrap();
            total *= my_ticket[idx] as u64;
        }
    }
    dbg!(total);
}

fn check_field(i: u32, f: &Field) -> bool {
    if (i >= f.a && i <= f.b) || (i >= f.c && i <= f.d) {
        return true;
    }
    return false;
}

fn valid(i: u32, fields: &[Field]) -> bool {
    for f in fields {
        if check_field(i, f) {
            return true;
        }
    }
    return false;
}
