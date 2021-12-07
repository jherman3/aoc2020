use aoc2020::read_file;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = read_file("inputs/p19.txt");
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap();
    let data = iter.next().unwrap();
    let mut rule_lines = HashMap::new();
    for l in rules.lines() {
        let mut sp = l.split(":");
        let i: u32 = sp.next().unwrap().parse().unwrap();
        let s = sp.next().unwrap().trim();
        rule_lines.insert(i, s);
    }
    let full_regex = format!("^{}$", build_regex("0", &rule_lines));
    let re = Regex::new(dbg!(&full_regex)).unwrap();
    let mut count = 0;
    for l in data.lines().map(|x| x.trim()) {
        if !l.is_empty() {
            if re.is_match(l) {
                count += 1;
            }
        }
    }
    dbg!(count);
}

fn build_single(pat: &str, map: &HashMap<u32, &str>) -> String {
    let mut res = String::new();
    for i in pat.split(" ") {
        if !i.is_empty() {
            res.push_str(&build_regex(map[&i.parse().unwrap()], map));
        }
    }
    return res;
}

fn build_regex(line: &str, map: &HashMap<u32, &str>) -> String {
    if line.starts_with('"') {
        return line[1..line.len()-1].into();
    } else {
        let pats: Vec<_> = line.split("|").collect();
        if pats.len() == 1 {
            return build_single(pats[0], map);
        } else {
            let joined = pats.iter().map(|x| build_single(x, map)).collect::<Vec<_>>().join("|");
            return format!("({})", joined);
        }
    }
}