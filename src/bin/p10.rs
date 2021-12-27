use aoc2020::each_line;
use aoc2020::AnyResult;

fn main() -> AnyResult<()> {
    let mut p1_total = 0;
    let mut p2_totals = Vec::new();
    for l in each_line("inputs/2021/p10.txt") {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for x in l.chars() {
            match x {
                '[' | '<' | '{' | '(' => stack.push(x),
                ')' => {
                    if stack.pop() != Some('(') {
                        p1_total += 3;
                        corrupt = true;
                        break;
                    }
                },
                ']' => {
                    if stack.pop() != Some('[') {
                        p1_total += 57;
                        corrupt = true;
                        break;
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        p1_total += 1197;
                        corrupt = true;
                        break;
                    }
                },
                '>' => {
                    if stack.pop() != Some('<') {
                        p1_total += 25137;
                        corrupt = true;
                        break;
                    }
                },
                _ => unreachable!(),
            }
        }
        if !corrupt {
            let mut line = 0u64;
            for c in stack.iter().rev() {
                line *= 5;
                line += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                }
            }
            p2_totals.push(line);
        }
    }
    dbg!(p1_total);
    p2_totals.sort();
    dbg!(p2_totals[p2_totals.len() / 2]);
    Ok(())
}
