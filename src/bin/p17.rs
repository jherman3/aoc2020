use std::cmp::Ordering;

fn main() {
    // small - target area: x=20..30, y=-10..-5
    // real - target area: x=48..70, y=-189..-148
    let (xmin, xmax, ymin, ymax) = (48, 70, -189, -148);
    let mut max_height = 0;
    let mut valid = 0;
    for vx in 0..200 {
        // given the input target must be in this rough range
        for vy in -200..300 {
            if let Some(height) = hit(xmin, xmax, ymin, ymax, vx, vy) {
                max_height = std::cmp::max(height, max_height);
                valid += 1;
            }
        }
    }
    dbg!(max_height, valid);
}

fn hit(xmin: i64, xmax: i64, ymin: i64, ymax: i64, mut vx: i64, mut vy: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut highest = 0;
    loop {
        x += vx;
        y += vy;
        highest = std::cmp::max(highest, y);
        match vx.cmp(&0) {
            Ordering::Less => vx += 1,
            Ordering::Greater => vx -= 1,
            _ => {}
        }
        vy -= 1;
        if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
            return Some(highest);
        }
        if vx <= 0 && x < xmin || vx >= 0 && x > xmax {
            return None;
        }
        if vy <= 0 && y < ymin {
            return None;
        }
    }
}
