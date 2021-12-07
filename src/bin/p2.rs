use aoc2020::each_line;
use aoc2020::AnyResult;

fn main() -> AnyResult<()> {
    let mut horizontal = 0;
    let mut depth = 0;
    for instruction in each_line("inputs/2021/p2.txt") {
        let (dir, end) = instruction.split_once(" ").ok_or("fail split")?;
        let num: u32 = end.parse()?;
        match dir {
            "forward" => horizontal += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => return Err("bad dir".into()),
        }
    }
    println!("p1 {}", horizontal * depth);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instruction in each_line("inputs/2021/p2.txt") {
        let (dir, end) = instruction.split_once(" ").ok_or("fail split")?;
        let num: u32 = end.parse()?;
        match dir {
            "forward" => {
                horizontal += num;
                depth += num * aim
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => return Err("bad dir".into()),
        }
    }
    println!("p2 {}", horizontal * depth);
    Ok(())
}
