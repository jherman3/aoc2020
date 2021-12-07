use aoc2020::parse_lines;

fn main() {
    let nums: Vec<u64> = parse_lines("inputs/p9.txt");

    // Part 1 - brute force
    let mut target = 0;
    for i in 25..nums.len() {
        if !check(nums[i], &nums[(i - 25)..i]) {
            target = nums[i];
            break;
        }
    }
    println!("p1: {}", target);

    // Part 2 - probably fast enough to brute force but
    // use 2 pointer approach
    // probably an edgecase somewhere but it gets the answer...
    let mut cur_total = 0;
    let mut left = 0;
    let mut right = 0;
    'outer: loop {
        while cur_total < target {
            cur_total += nums[right];
            right += 1;
        }
        if cur_total == target {
            break 'outer;
        }
        while cur_total > target {
            cur_total -= nums[left];
            left += 1;
        }
        if cur_total == target {
            break 'outer;
        }
    }
    // dbg!(left, right);
    let min = nums[left..right].iter().min().unwrap();
    let max = nums[left..right].iter().max().unwrap();
    println!("p2: {}", min + max);
}

fn check(n: u64, slice: &[u64]) -> bool {
    // n^2 brute force should be plenty fast since it's only 25^2 each time
    for i in 0..slice.len() {
        for j in 0..slice.len() {
            if i == j {
                continue;
            }
            if slice[i] + slice[j] == n {
                return true;
            }
        }
    }
    return false;
}
