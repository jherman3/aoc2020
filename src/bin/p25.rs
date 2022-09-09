use aoc2020::each_line;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Right,
    Down,
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for c in row {
            match c {
                Cell::Empty => print!("."),
                Cell::Right => print!(">"),
                Cell::Down => print!("v"),
            }
        }
        println!();
    }
}

fn main() {
    let mut grid = Vec::new();
    for l in each_line("inputs/2021/p25.txt") {
        let line = l
            .chars()
            .map(|x| match x {
                '.' => Cell::Empty,
                '>' => Cell::Right,
                'v' => Cell::Down,
                _ => panic!("invalid cell"),
            })
            .collect::<Vec<_>>();
        grid.push(line);
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut iters = 0;
    loop {
        iters += 1;
        let mut new_grid = vec![vec![Cell::Empty; n]; m];
        // First move east, copying other cells if necessary
        let mut updated = false;
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    Cell::Right => {
                        let target = (j + 1) % n;
                        if grid[i][target] == Cell::Empty {
                            updated = true;
                            new_grid[i][target] = Cell::Right;
                            new_grid[i][j] = Cell::Empty; // nop?
                        } else {
                            new_grid[i][j] = Cell::Right;
                        }
                    }
                    Cell::Down => {
                        new_grid[i][j] = Cell::Down;
                    }
                    Cell::Empty => {} // do nothing
                }
            }
        }
        grid = new_grid.clone();
        // Move down
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    Cell::Right => {}
                    Cell::Down => {
                        let target = (i + 1) % m;
                        if grid[target][j] == Cell::Empty {
                            updated = true;
                            new_grid[target][j] = Cell::Down;
                            new_grid[i][j] = Cell::Empty; // nop?
                        } else {
                            new_grid[i][j] = Cell::Down;
                        }
                    }
                    Cell::Empty => {} // do nothing
                }
            }
        }
        if !updated {
            break;
        }
        grid = new_grid;
    }
    print_grid(&grid);
    dbg!(iters);
}
