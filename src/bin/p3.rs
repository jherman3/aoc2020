use aoc2020::{line_input, AnyResult};

fn main() -> AnyResult<()> {
    let ls: Vec<String> = line_input("inputs/p3.txt")?;
    let bitmap: Vec<Vec<bool>> = ls
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    let slope = (1, 3);
    dbg!(get_count(&bitmap, slope));

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut total: u64 = 1;
    for slope in slopes {
        total *= get_count(&bitmap, slope) as u64;
    }
    dbg!(total);

    Ok(())
}

fn get_count(bitmap: &Vec<Vec<bool>>, slope: (usize, usize)) -> u32 {
    let cols = bitmap[0].len();
    let rows = bitmap.len();
    let mut xy = (0, 0);
    let mut count = 0;
    while xy.0 < rows {
        if bitmap[xy.0][xy.1] {
            count += 1;
        }
        xy.0 += slope.0;
        xy.1 = (xy.1 + slope.1) % cols;
    }
    count
}
