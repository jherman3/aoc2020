use aoc2020::read_file;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

fn print_grid(grid: &HashSet<Point>, xmax: u32, ymax: u32) {
    for y in 0..=ymax {
        for x in 0..=xmax {
            if grid.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input = read_file("inputs/2021/p13.txt");
    let (points, folds) = input.split_once("\n\n").unwrap();
    let mut grid = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<HashSet<_>>();

    let mut p1 = true;
    for fold in folds.lines() {
        let fold = &fold["fold along ".len()..];
        let (axis, amt) = fold.split_once("=").unwrap();
        let amt: u32 = amt.parse().unwrap();
        if axis == "x" {
            grid = grid.into_iter().map(|p| fold_along_x(p, amt)).collect();
        } else if axis == "y" {
            grid = grid.into_iter().map(|p| fold_along_y(p, amt)).collect();
        } else {
            panic!("bad axis");
        }
        if p1 {
            println!("Part 1: {}", grid.len());
            p1 = false;
        }
    }
    print_grid(&grid, 50, 7);
}

fn fold_along_x(p: Point, x: u32) -> Point {
    if p.x < x {
        p
    } else {
        Point {
            x: 2 * x - p.x,
            y: p.y,
        }
    }
}

fn fold_along_y(p: Point, y: u32) -> Point {
    if p.y < y {
        p
    } else {
        Point {
            y: 2 * y - p.y,
            x: p.x,
        }
    }
}
