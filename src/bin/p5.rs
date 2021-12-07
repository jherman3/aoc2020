use aoc2020::parse_lines;
use aoc2020::AnyResult;
use aoc2020_derive::regex_parsed;
use lazy_static::lazy_static;
use std::cmp::{max, min};

#[regex_parsed(r"(\d+),(\d+) -> (\d+),(\d+)")]
#[derive(Debug, Clone)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn main() -> AnyResult<()> {
    let input: Vec<Line> = parse_lines("inputs/2021/p5.txt");
    let max_x = input.iter().map(|x| max(x.x1, x.x2)).max().unwrap() as usize;
    let max_y = input.iter().map(|x| max(x.y1, x.y2)).max().unwrap() as usize;
    let mut board = vec![vec![0; max_y + 1]; max_x + 1];

    for l in &input {
        if l.x1 == l.x2 {
            let (y1, y2) = (min(l.y1, l.y2), max(l.y1, l.y2));
            for y in y1..=y2 {
                board[l.x1 as usize][y as usize] += 1;
            }
        } else if l.y1 == l.y2 {
            let (x1, x2) = (min(l.x1, l.x2), max(l.x1, l.x2));
            for x in x1..=x2 {
                board[x as usize][l.y1 as usize] += 1;
            }
        }
    }
    let p1 = board.iter().flatten().filter(|&&x| x > 1).count();
    dbg!(p1);
    for l in &input {
        // already counted
        if l.x1 == l.x2 || l.y1 == l.y2 {
            continue;
        }
        // either go up or down
        let Line {
            mut x1,
            mut x2,
            mut y1,
            mut y2,
        } = l;
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }
        let dir = if y1 < y2 { 1 } else { -1 };
        let mut y = y1;
        for x in x1..=x2 {
            board[x as usize][y as usize] += 1;
            y += dir;
        }
    }
    let p2 = board.iter().flatten().filter(|&&x| x > 1).count();
    dbg!(p2);
    // for i in 0..10 {
    //     for j in 0..10 {
    //         print!("{}", board[j][i]);
    //     }
    //     println!();
    // }

    Ok(())
}
