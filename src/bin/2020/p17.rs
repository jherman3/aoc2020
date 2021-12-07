use aoc2020::each_line;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut active = HashSet::new();
    let mut changeset = HashMap::new();
    for (x, l) in each_line("inputs/p17.txt").enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == '#' {
                active.insert((x as i32, y as i32, 0i32));
            }
        }
    }
    let mut cand = HashSet::new();
    for _iter in 0..6 {
        for &(x, y, z) in &active {
            for n in neighbors(x, y, z) {
                cand.insert(n);
            }
        }
        for &(x, y, z) in &cand {
            let c = count_neighbors(x, y, z, &active);
            let act = active.contains(&(x, y, z));
            if act && !(c == 2 || c == 3) {
                changeset.insert((x, y, z), false);
            } else if !act && c == 3 {
                changeset.insert((x, y, z), true);
            }
        }
        for (xyz, activate) in &changeset {
            if *activate {
                active.insert(*xyz);
            } else {
                active.remove(xyz);
            }
        }
        cand.clear();
        changeset.clear();
    }
    println!("p1: {}", active.len());
}

fn neighbors(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    let mut v = Vec::with_capacity(27);
    for i in (x - 1)..=x + 1 {
        for j in (y - 1)..=y + 1 {
            for k in (z - 1)..=z + 1 {
                v.push((i, j, k));
            }
        }
    }
    v
}

fn count_neighbors(x: i32, y: i32, z: i32, active: &HashSet<(i32, i32, i32)>) -> usize {
    let mut c = 0;
    for xyz in neighbors(x, y, z) {
        if !(xyz == (x, y, z)) && active.contains(&xyz) {
            c += 1;
        }
    }
    c
}
