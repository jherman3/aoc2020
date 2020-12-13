use aoc2020::parse_lines;

fn main() {
    let mut nums: Vec<u32> = parse_lines("inputs/p10.txt");
    nums.push(0);  // input device
    nums.sort();
    nums.push(nums[nums.len()-1] + 3); // Three higher than max
    let diffs = nums.iter().zip(&nums[1..]).map(|(x, y)| y - x);
    let mut ones = 0;
    let mut threes = 0;
    for d in diffs {
        if d == 1 {
            ones += 1;
        }
        if d == 3 {
            threes += 1;
        }
    }
    println!("p1: {}", ones * threes);
    let mut dp = vec![0u64; nums.len()];
    dp[0] = 1;
    for i in 1..nums.len() {
        let cur = nums[i];
        let mut cand = i as i32 - 1;
        while cand >= 0 && cur - nums[cand as usize] <= 3 {
            dp[i] += dp[cand as usize];
            cand -= 1;
        }
    }
    println!("p2: {}", dp[dp.len() - 1]);
}
