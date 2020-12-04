use aoc2020::line_input;
use aoc2020::AnyResult;

use regex::Regex;

fn main() -> AnyResult<()> {
    let lines: Vec<String> = line_input("inputs/p2.txt")?;
    let reg = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let mut valid_p1 = 0;
    let mut valid_p2 = 0;
    for l in lines {
        let caps = reg.captures(&l).unwrap();
        let u1: usize = caps.get(1).unwrap().as_str().parse()?;
        let u2: usize = caps.get(2).unwrap().as_str().parse()?;
        let c: char = caps.get(3).unwrap().as_str().parse()?;
        let pass: &str = caps.get(4).unwrap().as_str();
        let cnt = pass.chars().filter(|&x| x == c).count();
        if cnt >= u1 && cnt <= u2 {
            valid_p1 += 1;
        }
        let b = pass.as_bytes();
        let cb = c as u8;
        if (b[u1 - 1] == cb || b[u2 - 1] == cb) && !(b[u1 - 1] == cb && b[u2 - 1] == cb) {
            valid_p2 += 1;
        }
    }
    dbg!(valid_p1, valid_p2);
    Ok(())
}
