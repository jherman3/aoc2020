use aoc2020::read_file;
use std::collections::HashSet;


fn find_deltas(beacons: &[Point], r: Rotation) -> Vec<Vec<Point>> {
    let mut deltas = Vec::with_capacity(beacons.len());
    for p1 in beacons {
        let p1 = r.rotate(*p1);
        let mut beacon_deltas = Vec::with_capacity(beacons.len()-1);
        for p2 in beacons {
            let p2 = r.rotate(*p2);
            if p1 != p2 {
                beacon_deltas.push(Point(p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2));
            }
        }
        //beacon_deltas.sort();
        deltas.push(beacon_deltas);
    }
    //deltas.sort();
    deltas
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rotation(i32);  // 0 - 23, representing number of 90 deg turns

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point(i32, i32, i32);

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Rotation {
    pub fn rotate(self, p: Point) -> Point {
        // first select which face is up (z-axis).
        let p = match self.0 / 4 {
            0 => Point(p.0, p.1, p.2),
            1 => Point(p.1, p.2, p.0),
            2 => Point(p.2, p.0, p.1),
            3 => Point(-p.1, -p.0, -p.2),
            4 => Point(-p.2, -p.1, -p.0),
            5 => Point(-p.0, -p.2, -p.1),
            _ => panic!("invalid rotation"),
        };
        // rotate all 4 options with z-axis pointing up
        match self.0 % 4 {
            0 => Point(p.0, p.1, p.2),
            1 => Point(p.1, -p.0, p.2),
            2 => Point(-p.0, -p.1, p.2),
            3 => Point(-p.1, p.0, p.2),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = read_file("inputs/2021/p19_orient.txt");
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    for scan in input.split("\n\n") {
        let mut lines = scan.lines();
        lines.next();
        let mut beacons= Vec::new();
        for l in lines {
            let nums: Vec<_> = l.split(",").collect();
            beacons.push(Point(nums[0].parse().unwrap(), nums[1].parse().unwrap(), nums[2].parse().unwrap()));
        }
        scanners.push(beacons);
    }

    let base = &scanners[0];
    let z = &scanners[1];
    let mut base_deltas = find_deltas(base, Rotation(0));
    for rot in 0..24 {
        let mut z_deltas = find_deltas(z, Rotation(rot));
        if z_deltas == base_deltas {
            dbg!(rot);
        }
        //dbg!(&base_deltas, &z_deltas);
    }

}
