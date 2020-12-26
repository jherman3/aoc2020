use std::collections::{HashSet, HashMap};
use aoc2020::each_line;

fn main() {
    let mut active = HashSet::new();
    let mut changeset = HashMap::new();
    for (x, l) in each_line("inputs/p17.txt").enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == '#' {
                active.insert((x as i32, y as i32, 0i32, 0i32));
            }
        }
    }
    let mut cand = HashSet::new();
    for _iter in 0..6 {
        for &(x, y, z, w) in &active {
            for n in neighbors(x, y, z, w) {
                cand.insert(n);
            }
        }
        for &(x, y, z, w) in &cand {
            let c = count_neighbors(x, y, z, w, &active);
            let act = active.contains(&(x, y, z, w));
            if act && !(c == 2 || c == 3) {
                changeset.insert((x, y, z, w), false);
            } else if !act && c == 3 {
                changeset.insert((x, y, z, w), true);
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
    println!("p2: {}", active.len());
}

fn neighbors(x: i32, y: i32, z: i32, w: i32) -> Vec<(i32, i32, i32, i32)> {
    let mut v = Vec::with_capacity(81);
    for i in (x-1)..=x+1 {
        for j in (y-1)..=y+1 {
            for k in (z-1)..=z+1 {
                for l in (w-1)..=w+1 {
                    v.push((i, j, k, l));
                }
            }
        }
    }
    v
}

fn count_neighbors(x: i32, y: i32, z: i32, w: i32, active: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let mut c = 0;
    for xyz in neighbors(x, y, z, w) {
        if !(xyz == (x, y, z, w)) && active.contains(&xyz) {
            c += 1;
        }
    }
    c
}