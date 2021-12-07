use aoc2020::each_line;

fn p1() {
    // (+north, +east)
    let mut pos = (0i32, 0i32);
    let mut angle = 0i32;
    // E, S, W, N -> +1 = turn 90 degrees right
    const DIRS: &[(i32, i32)] = &[(0, 1), (-1, 0), (0, -1), (1, 0)];
    for l in each_line("inputs/p12.txt") {
        let (dir, s_amt) = l.split_at(1);
        let amt: i32 = s_amt.parse().unwrap();
        let change = match dir {
            "N" => DIRS[3],
            "S" => DIRS[1],
            "E" => DIRS[0],
            "W" => DIRS[2],
            "R" => {
                angle = (angle + amt / 90) % 4;
                (0, 0)
            }
            "L" => {
                angle = (angle + 4 - amt / 90) % 4;
                (0, 0)
            }
            "F" => DIRS[angle as usize],
            _ => panic!("Invalid"),
        };
        pos.0 += change.0 * amt;
        pos.1 += change.1 * amt;
    }
    let dist = pos.0.abs() + pos.1.abs();
    println!("p1: {}", dist);
}

fn rot(p: (i32, i32), ang: i32) -> (i32, i32) {
    match ang {
        0 => p,
        90 => (-p.1, p.0),
        180 => (-p.0, -p.1),
        270 => (p.1, -p.0),
        _ => panic!("bad angle"),
    }
}

fn p2() {
    // (+north, +east)
    let mut pos = (0i32, 0i32);
    let mut waypoint = (1i32, 10i32);
    for l in each_line("inputs/p12.txt") {
        let (dir, s_amt) = l.split_at(1);
        let amt: i32 = s_amt.parse().unwrap();
        match dir {
            "N" => {
                waypoint.0 += amt;
            }
            "S" => {
                waypoint.0 -= amt;
            }
            "E" => {
                waypoint.1 += amt;
            }
            "W" => {
                waypoint.1 -= amt;
            }
            "R" => {
                waypoint = rot(waypoint, amt);
            }
            "L" => {
                waypoint = rot(waypoint, (360 - amt) % 360);
            }
            "F" => {
                pos.0 += waypoint.0 * amt;
                pos.1 += waypoint.1 * amt;
            }
            _ => panic!("Invalid"),
        };
    }
    let dist = pos.0.abs() + pos.1.abs();
    println!("p2: {}", dist);
}

fn main() {
    p1();
    p2();
}
