use aoc2020::parse_lines;
use aoc2020::AnyResult;

// input is small, brute force works for both
fn main() -> AnyResult<()> {
    let nums: Vec<u32> = parse_lines("inputs/p1.txt");
    'outer: for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] + nums[j] == 2020 {
                println!("{}", nums[i] * nums[j]);
                break 'outer;
            }
        }
    }

    'outer2: for i in 0..nums.len() {
        for j in 0..i {
            for k in 0..j {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("{}", nums[i] * nums[j] * nums[k]);
                    break 'outer2;
                }
            }
        }
    }
    return Ok(());
}
