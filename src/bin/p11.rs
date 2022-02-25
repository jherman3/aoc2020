#![allow(clippy::needless_range_loop)]
use aoc2020::{each_line, AnyResult, Neighbors};

fn main() -> AnyResult<()> {
    let mut board: Vec<Vec<u8>> = each_line("inputs/2021/p11.txt")
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8).collect())
        .collect();
    let m = board.len();
    let n = board[0].len();
    let mut flashes = 0;
    for step in 0..1000 {
        let mut flashed = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                board[i][j] += 1;
            }
        }
        let mut updated = true;
        while updated {
            updated = false;
            for i in 0..m {
                for j in 0..n {
                    if board[i][j] > 9 && !flashed[i][j] {
                        flashes += 1;
                        flashed[i][j] = true;
                        updated = true;
                        for (x, y) in Neighbors::new(i, j, m, n) {
                            board[x][y] += 1;
                        }
                    }
                }
            }
        }
        // reset to 0 *after* all flashes
        let mut all = true;
        for i in 0..m {
            for j in 0..n {
                if flashed[i][j] {
                    board[i][j] = 0;
                } else {
                    all = false;
                }
            }
        }
        if all {
            dbg!(step + 1);
            break;
        }
        if step == 99 {
            dbg!(flashes);
        }
    }
    Ok(())
}
