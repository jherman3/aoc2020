use std::collections::{HashMap, HashSet};

use aoc2020::{each_line, AnyResult};

fn main() -> AnyResult<()> {
    let mut p1_count = 0;
    for l in each_line("inputs/2021/p8.txt") {
        let (_start, end) = l.split_once('|').expect("once");
        for w in end.split(' ') {
            if [2, 3, 4, 7].contains(&w.len()) {
                p1_count += 1;
            }
        }
    }
    dbg!(p1_count);

    let mut p2_sum: usize = 0;
    for l in each_line("inputs/2021/p8.txt") {
        let (keys, code) = l.split_once('|').expect("once");
        let mut individual: Vec<HashSet<char>> = keys
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect())
            .collect();
        // we get 1, 7, 4, 8 for free
        // 0: abcefg
        // 1: cf
        // 2: acdeg
        // 3: acdfg
        // 4: bcdf
        // 5: abdfg
        // 6: abdefg
        // 7: acf
        // 8: abcdefg
        // 9: abcdfg
        let mut digits: [Option<HashSet<char>>; 10] = Default::default();
        digits[1] = find_remove(&mut individual, |x| x.len() == 2);
        digits[7] = find_remove(&mut individual, |x| x.len() == 3);
        digits[4] = find_remove(&mut individual, |x| x.len() == 4);
        digits[8] = find_remove(&mut individual, |x| x.len() == 7);
        // 0 and 9 entirely contain 7, 6 does not.
        digits[6] = find_remove(&mut individual, |x| {
            x.len() == 6 && !digits[7].as_ref().unwrap().is_subset(x)
        });
        // 9 entirely contains 4
        digits[9] = find_remove(&mut individual, |x| {
            x.len() == 6 && digits[4].as_ref().unwrap().is_subset(x)
        });
        // remaining has to be 0
        digits[0] = find_remove(&mut individual, |x| x.len() == 6);
        // 2 3 5
        // 3 is only one that contains 1
        digits[3] = find_remove(&mut individual, |x| {
            digits[1].as_ref().unwrap().is_subset(x)
        });
        // for 2, union with 6 creates 8
        digits[2] = find_remove(&mut individual, |x| {
            digits[6].as_ref().unwrap().union(x).count() == 7
        });
        digits[5] = individual.pop();
        let mut digit_mapping: HashMap<String, usize> = HashMap::new();
        for (x, key) in digits.iter_mut().enumerate() {
            let mut key: Vec<char> = key.take().expect("unsolved").into_iter().collect();
            key.sort_unstable();
            digit_mapping.insert(key.into_iter().collect(), x);
        }
        let mut cur = String::new();
        for digit in code.split(' ') {
            if digit.is_empty() {
                continue;
            }
            let mut key: Vec<char> = digit.chars().collect();
            key.sort_unstable();
            let key_str: String = key.iter().collect();
            cur.push_str(&digit_mapping[&key_str].to_string());
        }
        p2_sum += cur.parse::<usize>().unwrap();
    }
    dbg!(p2_sum);
    Ok(())
}

fn find_remove<T, F: Fn(&T) -> bool>(v: &mut Vec<T>, f: F) -> Option<T> {
    let i = v.iter().position(f)?;
    Some(v.swap_remove(i))
}
