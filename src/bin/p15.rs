use aoc2020::each_line;
use std::collections::BinaryHeap;

fn main() {
    let grid: Vec<Vec<i32>> = each_line("inputs/2021/p15.txt")
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
        .collect();
    let m = grid.len();
    let n = grid[0].len();
    let p1 = shortest_path_len(&grid);
    dbg!(p1);
    let mut grid2 = vec![vec![0; 5 * n]; 5 * m];
    for i in 0..grid2.len() {
        for j in 0..grid2[0].len() {
            grid2[i][j] = grid[i % m][j % n] + (i / m + j / n) as i32;
            while grid2[i][j] > 9 {
                grid2[i][j] -= 9; // because we dont allow 0 - this is like mod 9 but not quite;
            }
        }
    }
    let p2 = shortest_path_len(&grid2);
    dbg!(p2);
}

fn shortest_path_len(grid: &[Vec<i32>]) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut to_visit = BinaryHeap::new(); // (cost, x, y)
    let mut min_cost = vec![vec![i32::MAX; n]; m];
    min_cost[0][0] = 0;
    // dijkstras
    to_visit.push((0, 0, 0));
    while let Some((cost, x, y)) = to_visit.pop() {
        let cost = -cost; // negate because std is max heap
        if (x, y) == (m as i32 - 1, n as i32 - 1) {
            return cost;
        }
        if min_cost[x as usize][y as usize] < cost {
            // already found better path
            continue;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let xp = x + dx;
            let yp = y + dy;
            if xp >= 0 && yp >= 0 && xp < m as i32 && yp < n as i32 {
                let xp = xp as usize;
                let yp = yp as usize;
                let new_cost = grid[xp][yp] + cost;
                if new_cost < min_cost[xp][yp] {
                    min_cost[xp][yp] = new_cost;
                    to_visit.push((-new_cost, xp as i32, yp as i32));
                }
            }
        }
    }
    0
}
