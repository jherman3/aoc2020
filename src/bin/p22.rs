use aoc2020::each_line;
use std::cmp::{max, min};
use std::collections::HashMap;

fn parse_range(range: &str) -> (i32, i32) {
    let (l, r) = range.split_once("..").unwrap();
    (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap() + 1)
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Cube {
    // (start, end) endpoints, semi-open.
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

fn overlap_1d(mut l1: (i32, i32), mut l2: (i32, i32)) -> Option<(i32, i32)> {
    /// Return overlap of lines l1 and l2 - right endpoint is exclusive
    if l1.0 > l2.0 {
        (l1, l2) = (l2, l1);
    }
    // now l1 is before l2
    let left_bound = max(l2.0, l1.0);
    let right_bound = min(l1.1, l2.1);
    if left_bound < right_bound {
        Some((left_bound, right_bound))
    } else {
        None
    }
}

impl Cube {
    fn overlap(&self, other: &Cube) -> Option<Cube> {
        let dx = overlap_1d(self.x, other.x)?;
        let dy = overlap_1d(self.y, other.y)?;
        let dz = overlap_1d(self.z, other.z)?;
        Some(Cube {
            x: dx,
            y: dy,
            z: dz,
        })
    }

    fn volume(&self) -> usize {
        (self.x.1 - self.x.0) as usize
            * (self.y.1 - self.y.0) as usize
            * (self.z.1 - self.z.0) as usize
    }
}

fn main() {
    let mut history: HashMap<Cube, i32> = HashMap::new();
    for line in each_line("inputs/2021/p22.txt") {
        let (action, rest) = line.split_once(" ").unwrap();
        let mut xyz = rest.split(",");
        let x = xyz.next().unwrap();
        let xr = parse_range(&x[2..]);
        let y = xyz.next().unwrap();
        let yr = parse_range(&y[2..]);
        let z = xyz.next().unwrap();
        let zr = parse_range(&z[2..]);
        // part 1
        // if [xr.0, xr.1, yr.0, yr.1, zr.0, zr.1].map(|x| x.abs()).iter().max().unwrap() > &50 {
        //     continue;
        // }
        let new_cube = Cube {
            x: xr,
            y: yr,
            z: zr,
        };
        let mut to_add = Vec::new();
        for (c, existing) in &history {
            // cancel out existing overlaps and replace with new entry
            if let Some(o) = c.overlap(&new_cube) {
                to_add.push((o, -existing));
            }
        }
        if action == "on" {
            to_add.push((new_cube, 1));
        }
        for (c, v) in to_add {
            *history.entry(c).or_default() += v
        }
    }
    let mut sum: i64 = 0;
    for (c, h) in history {
        sum += c.volume() as i64 * h as i64;
    }
    dbg!(sum);
}
