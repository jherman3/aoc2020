#![feature(iter_intersperse)]

use aoc2020::each_line;
use aoc2020::AnyResult;
use std::collections::HashSet;

fn main() -> AnyResult<()> {
    let mut lines = each_line("inputs/2021/p4.txt");
    let nums: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();
    let rem: String = lines.intersperse("\n".to_string()).collect();
    let mut boards = Vec::new();
    for board in rem.split("\n\n") {
        let entries: Vec<u8> = board
            .split_whitespace()
            .map(|x| x.parse().expect("board"))
            .collect();
        boards.push(entries);
    }
    let mut called = HashSet::new();
    let mut first = None;
    let mut last = None;
    for x in nums {
        called.insert(x);
        let mut to_remove = Vec::new();
        for (i, board) in boards.iter().enumerate() {
            if check_board(board, &called) {
                if first.is_none() {
                    first = Some(score_board(board, &called, x));
                }
                last = Some(score_board(board, &called, x));
                to_remove.push(i);
            }
        }
        for &x in to_remove.iter().rev() {
            boards.remove(x);
        }
    }
    println!("p1 {} p2 {}", first.unwrap(), last.unwrap());
    Ok(())
}

fn score_board(board: &[u8], called: &HashSet<u8>, num: u8) -> u32 {
    let sum: u32 = board
        .iter()
        .filter(|&x| !called.contains(x))
        .map(|&x| x as u32)
        .sum();
    sum * (num as u32)
}

fn check_board(board: &[u8], called: &HashSet<u8>) -> bool {
    for offset in 0..5 {
        if (0..5).all(|col| called.contains(&board[offset * 5 + col])) {
            return true;
        }
        if (0..5).all(|row| called.contains(&board[row * 5 + offset])) {
            return true;
        }
    }
    // first diag
    if (0..5).all(|i| called.contains(&board[i * 5 + i])) {
        return true;
    }
    // other diag
    if (0..5).all(|i| called.contains(&board[(4 - i) * 5 + 4 - i])) {
        return true;
    }
    false
}
