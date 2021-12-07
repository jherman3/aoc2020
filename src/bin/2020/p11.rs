#![feature(destructuring_assignment)]

use aoc2020::each_line;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Occupied,
    Empty,
    Floor,
}

fn conv(c: Cell) -> char {
    match c {
        Cell::Occupied => '#',
        Cell::Empty => 'L',
        Cell::Floor => '.',
    }
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<Cell>>) {
    let s = board
        .iter()
        .map(|c| c.iter().map(|x| conv(*x)).collect::<String>());
    for r in s {
        println!("{}", r);
    }
    println!();
}

fn main() {
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for l in each_line("inputs/p11.txt") {
        board.push(
            l.chars()
                .map(|c| if c == 'L' { Cell::Empty } else { Cell::Floor })
                .collect(),
        );
    }

    println!("p1: {}", solve(&board, count_adj_p1, 4));
    println!("p2: {}", solve(&board, count_adj_p2, 5));
}

fn solve<F: Fn(&Vec<Vec<Cell>>, usize, usize) -> u32 + Copy>(
    board: &Vec<Vec<Cell>>,
    counter: F,
    lim: u32,
) -> usize {
    let mut changed = true;
    let mut board = board.clone(); // unnecessary...
    while changed {
        (board, changed) = iterate(&board, counter, lim);
    }
    let p: usize = board
        .iter()
        .map(|b| b.iter().filter(|&&c| c == Cell::Occupied).count())
        .sum();
    p
}

const DIRS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adj_p2(board: &Vec<Vec<Cell>>, i: usize, j: usize) -> u32 {
    let mut cnt = 0;
    let m = board.len() as i32;
    let n = board[0].len() as i32;

    for d in DIRS {
        let mut a = i as i32 + d.0;
        let mut b = j as i32 + d.1;
        while a >= 0 && a < m && b >= 0 && b < n {
            match board[a as usize][b as usize] {
                Cell::Occupied => {
                    cnt += 1;
                    break;
                }
                Cell::Empty => {
                    break;
                }
                Cell::Floor => {
                    a += d.0;
                    b += d.1;
                }
            }
        }
    }
    cnt
}

fn count_adj_p1(board: &Vec<Vec<Cell>>, i: usize, j: usize) -> u32 {
    let mut cnt = 0;
    let m = board.len() as i32;
    let n = board[0].len() as i32;

    for d in DIRS {
        let a = i as i32 + d.0;
        let b = j as i32 + d.1;
        if a >= 0 && a < m && b >= 0 && b < n && board[a as usize][b as usize] == Cell::Occupied {
            cnt += 1;
        }
    }
    cnt
}

fn iterate<F: Fn(&Vec<Vec<Cell>>, usize, usize) -> u32 + Copy>(
    board: &Vec<Vec<Cell>>,
    counter: F,
    lim: u32,
) -> (Vec<Vec<Cell>>, bool) {
    let mut new = board.clone();
    let mut changed = false;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            new[i][j] = match board[i][j] {
                Cell::Floor => Cell::Floor,
                x => {
                    let c = counter(board, i, j);
                    if x == Cell::Occupied && c >= lim {
                        changed = true;
                        Cell::Empty
                    } else if x == Cell::Empty && c == 0 {
                        changed = true;
                        Cell::Occupied
                    } else {
                        x
                    }
                }
            }
        }
    }
    (new, changed)
}
