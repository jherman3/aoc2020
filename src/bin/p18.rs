use aoc2020::each_line;

#[derive(Clone, Debug)]
struct Num {
    value: u32,
    depth: u8,
}

fn parse_line(line: &str) -> Vec<Num> {
    let mut res = Vec::new();
    let mut depth = 0;
    for c in line.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {}
            x => res.push(Num {
                value: x.to_digit(10).expect("digit"),
                depth,
            }),
        }
    }
    assert_eq!(depth, 0);
    res
}

fn explode(nums: &mut Vec<Num>) -> bool {
    if let Some(idx) = nums.iter().position(|x| x.depth == 5) {
        assert!(idx < nums.len() - 1);
        if idx > 0 {
            nums[idx - 1].value += nums[idx].value;
        }
        if idx < nums.len() - 2 {
            nums[idx + 2].value += nums[idx + 1].value;
        }
        nums.remove(idx + 1);
        nums[idx].value = 0;
        nums[idx].depth -= 1;
        true
    } else {
        false
    }
}

fn split(nums: &mut Vec<Num>) -> bool {
    if let Some(idx) = nums.iter().position(|x| x.value > 9) {
        let value = nums[idx].value;
        let depth = nums[idx].depth;
        nums[idx] = Num {
            value: value / 2,
            depth: depth + 1,
        };
        nums.insert(
            idx + 1,
            Num {
                value: (value + 1) / 2,
                depth: depth + 1,
            },
        );
        true
    } else {
        false
    }
}

fn add_assign(num1: &mut Vec<Num>, num2: &[Num]) {
    for x in num1.iter_mut() {
        x.depth += 1;
    }
    for mut x in num2.iter().cloned() {
        x.depth += 1;
        num1.push(x);
    }
    let mut changed = true;
    while changed {
        changed = explode(num1);
        if !changed {
            changed = split(num1);
        }
    }
}

fn magnitude(nums: &[Num]) -> u32 {
    let mut nums = nums.to_vec();
    while nums.len() > 1 {
        // todo n^2
        let mut max_depth = 0;
        let mut idx = 0;
        for (i, x) in nums.iter().enumerate() {
            if x.depth > max_depth {
                max_depth = x.depth;
                idx = i;
            }
        }
        nums[idx].depth -= 1;
        nums[idx].value = nums[idx].value * 3 + nums[idx + 1].value * 2;
        nums.remove(idx + 1);
    }
    return nums[0].value;
}

fn main() {
    let mut it = each_line("inputs/2021/p18.txt");
    let mut nums = parse_line(&it.next().unwrap());
    while let Some(line) = it.next() {
        add_assign(&mut nums, &parse_line(&line));
    }
    dbg!(magnitude(&nums));
    let mut max_magnitude = 0;
    let all_nums: Vec<_> = each_line("inputs/2021/p18.txt").map(|x| parse_line(&x)).collect();
    for i in 0..all_nums.len() {
        for j in 0..all_nums.len() {
            if i == j {
                continue;
            }
            let mut n = all_nums[i].clone();
            add_assign(&mut n, &all_nums[j]);
            max_magnitude = std::cmp::max(max_magnitude, magnitude(&n));
        }
    }
    dbg!(max_magnitude);
}
