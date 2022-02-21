#![feature(array_windows)]

use aoc2020::read_file;
use std::collections::HashMap;

fn main() {
    let input = read_file("inputs/2021/p14.txt");
    let (polymer, mapping) = input.split_once("\n\n").unwrap();
    let mut insertions = HashMap::new();
    for m in mapping.lines() {
        let (pair, replacement) = m.split_once(" -> ").unwrap();
        let mut it = pair.chars();
        insertions.insert(
            (it.next().unwrap(), it.next().unwrap()),
            replacement.chars().next().unwrap(),
        );
    }
    let chars: Vec<_> = polymer.chars().collect();

    let p1 = solve(&chars, &insertions, 10);
    let p2 = solve(&chars, &insertions, 40);
    dbg!(p1, p2);
}

fn solve(chars: &[char], insertions: &HashMap<(char, char), char>, iterations: usize) -> usize {
    let mut pairs = HashMap::new();
    for [c1, c2] in chars.array_windows::<2>() {
        *pairs.entry((*c1, *c2)).or_insert(0usize) += 1;
    }
    for _iteration in 0..iterations {
        let mut new_pairs = HashMap::new();
        for (pair, count) in pairs.into_iter() {
            if let Some(&rep) = insertions.get(&pair) {
                *new_pairs.entry((pair.0, rep)).or_insert(0) += count;
                *new_pairs.entry((rep, pair.1)).or_insert(0) += count;
            } else {
                *new_pairs.entry(pair).or_insert(0) += count;
            }
        }
        pairs = new_pairs;
    }
    let mut freqs = HashMap::new();
    for ((c1, c2), count) in pairs {
        *freqs.entry(c1).or_insert(0) += count;
        *freqs.entry(c2).or_insert(0) += count;
    }
    // first and last aren't double counted so increment
    *freqs.get_mut(chars.first().unwrap()).unwrap() += 1;
    *freqs.get_mut(chars.last().unwrap()).unwrap() += 1;
    for v in freqs.values_mut() {
        *v /= 2;
    }
    let most = freqs.values().max().unwrap();
    let least = freqs.values().min().unwrap();
    most - least
}
