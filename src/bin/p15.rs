use std::collections::HashMap;

const INPUT: &[usize] = &[12, 1, 16, 3, 11, 0];
// const INPUT: &[usize] = &[0, 3, 6];

fn main() {
    println!("p1: {}", run(2020));
    println!("p1: {}", run(30_000_000));
}

fn run(max: usize) -> usize {
    // num -> (most recent, next most recent)
    let mut turns: HashMap<usize, (usize, Option<usize>)> = HashMap::new();

    let mut prev = 0;
    for (turn, x) in INPUT.iter().enumerate() {
        if let Some(&(older, _)) = turns.get(x) {
            // we've seen it before so update
            turns.insert(*x, (turn, Some(older)));
        } else {
            turns.insert(*x, (turn, None));
        }
        prev = *x;
    }

    for turn in INPUT.len()..max {
        if let &(last, Some(older)) = turns.get(&prev).unwrap() {
            prev = last - older;
        } else {
            prev = 0;
        }
        if let Some(&(oldval, _)) = turns.get(&prev) {
            turns.insert(prev, (turn, Some(oldval)));
        } else {
            turns.insert(prev, (turn, None));
        }
    }
    prev
}
