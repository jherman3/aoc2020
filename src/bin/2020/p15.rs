use std::collections::HashMap;

const INPUT: &[usize] = &[12, 1, 16, 3, 11, 0];
// const INPUT: &[usize] = &[0, 3, 6];

fn main() {
    println!("p1: {}", run(2020));
    println!("p1: {}", run(30_000_000));
}

fn run(max: usize) -> usize {
    // Num -> last spoken
    let mut turns: HashMap<usize, usize> = HashMap::new();

    let mut prev = 0;
    for (turn, x) in INPUT.iter().enumerate() {
        turns.insert(*x, turn);
        prev = *x;
    }

    for turn in INPUT.len()..max {
        let cur;
        if let Some(t) = turns.get(&prev) {
            cur = turn - 1 - t;
        } else {
            cur = 0;
        };
        turns.insert(prev, turn - 1);
        prev = cur;
    }
    prev
}
