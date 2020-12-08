#![feature(str_split_once)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate aoc2020_derive;

use std::collections::{HashMap, HashSet, VecDeque};

use aoc2020::each_line;

#[regex_parsed(r"(\d+) (.+) bags?\.?")]
#[derive(Debug)]
struct BagSpec {
    count: usize,
    color: String,
}

fn main() {
    let mut counts: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut inverse: HashMap<String, HashSet<String>> = HashMap::new();
    for l in each_line("inputs/p7.txt") {
        let (col, rest) = l.split_once("bags contain").unwrap();
        let col = col.trim();
        let nodes = counts.entry(col.into()).or_default();
        for contains in rest.split(",") {
            if contains.trim() == "no other bags." {
                continue;
            }
            let bs: BagSpec = contains.trim().parse().unwrap();
            nodes.insert(bs.color.clone(), bs.count);
            inverse.entry(bs.color).or_default().insert(col.into());
        }
    }
    // dbg!(counts);
    let root = "shiny gold";
    let mut leaves = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_back() {
        leaves.insert(node);
        if let Some(children) = inverse.get(node) {
            for k in children {
                queue.push_back(k);
            }
        }
    }
    let p1 = leaves.len() - 1;

    fn dfs(
        node: &str,
        counts: &HashMap<String, HashMap<String, usize>>,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        if let Some(c) = cache.get(node) {
            return *c;
        }
        let mut total = 1;
        if let Some(children) = counts.get(node) {
            for (child, mult) in children.iter() {
                total += mult * dfs(child, counts, cache);
            }
        }
        cache.insert(node.into(), total);
        total
    };
    let mut cache: HashMap<String, usize> = HashMap::new();
    let p2 = dfs(root, &counts, &mut cache) - 1;
    dbg!(p1, p2);
}
