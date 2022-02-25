use aoc2020::each_line;
use std::collections::{BinaryHeap, HashMap};

fn neighbors(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    // TODO faster without allocations/generator maybe
    let mut out = vec![];
    let i = i as i32;
    let j = j as i32;
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = i + dx;
        let y = j + dy;
        if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
            out.push((x as usize, y as usize));
        }
    }
    out
}

fn main() {
    let map: Vec<Vec<u8>> = each_line("inputs/2021/p9.txt")
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect();
    let n = map.len();
    let m = map[0].len();
    let mut p1_total: u64 = 0;
    for i in 0..n {
        for j in 0..m {
            let low = !neighbors(i, j, n, m)
                .iter()
                .any(|&(x, y)| map[x][y] <= map[i][j]);
            if low {
                p1_total += map[i][j] as u64 + 1;
            }
        }
    }
    dbg!(p1_total);

    let mut flood_map = vec![vec![0; m as usize]; n as usize];
    let mut basin_id = 1;
    // flood fill out from nines

    fn explore(
        map: &[Vec<u8>],
        flood_map: &mut Vec<Vec<i32>>,
        basin_id: i32,
        i: usize,
        j: usize,
        n: usize,
        m: usize,
    ) {
        for (x, y) in neighbors(i, j, n, m) {
            if flood_map[x][y] != 0 || map[x][y] == 9 {
                continue;
            }
            flood_map[x][y] = basin_id;
            explore(map, flood_map, basin_id, x, y, n, m);
        }
    }

    for i in 0..n {
        for j in 0..m {
            if flood_map[i as usize][j as usize] != 0 || map[i as usize][j as usize] == 9 {
                continue;
            }
            explore(&map, &mut flood_map, basin_id, i, j, n, m);
            basin_id += 1;
        }
    }
    let mut counts = HashMap::new();
    for x in flood_map.iter().flatten() {
        *counts.entry(x).or_insert(0usize) += 1;
    }
    let mut counts_heap = BinaryHeap::new();
    for (&id, count) in counts {
        if id != 0 {
            counts_heap.push((count, id));
        }
    }
    let mut p2_prod = 1;
    for _ in 0..3 {
        p2_prod *= counts_heap.pop().unwrap().0;
    }
    dbg!(p2_prod);
}
