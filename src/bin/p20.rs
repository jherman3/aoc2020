use aoc2020::{each_line, Neighbors};

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &b in row {
            if b {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let mut input = each_line("inputs/2021/p20.txt");
    let mapping: Vec<bool> = input.next().unwrap().chars().map(|y| y == '#').collect();
    input.next().unwrap();
    let mut grid: Vec<Vec<bool>> = input.map(|x| x.chars().map(|y| y == '#').collect()).collect();
    // print_grid(&grid);
    // println!();
    // outside ring will toggle on and off
    let mut outer = false;
    for _ in 0..50 {
        let m = grid.len();
        let n = grid[0].len();
        let mut new_grid = vec![vec![false; n+6]; m+6];
        let m = m as i32;
        let n = n as i32;
        for i in 0..m+6 {
            for j in 0..n+6 {
                let mut cur_val = 0;
                let mut shift_counter = 8;
                for dx in -1..=1i32 {
                    for dy in -1..=1i32 {
                        let x = i + dx - 3;
                        let y = j + dy - 3;
                        let testval;
                        if x >= 0 && y >= 0 && x < m && y < n {
                            testval = grid[x as usize][y as usize];
                        } else {
                            testval = outer;
                        }
                        if testval {
                            cur_val += 1 << shift_counter;
                        }
                        shift_counter -= 1;
                    }
                }
                new_grid[i as usize][j as usize] = mapping[cur_val as usize];
            }
        }
        grid = new_grid;
        outer = !outer;
        // print_grid(&grid);
        // println!();
    }
    let p1_count = grid.iter().flatten().filter(|x| **x).count();
    dbg!(p1_count);
}