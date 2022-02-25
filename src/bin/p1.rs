use aoc2020::parse_lines;

fn count_increasing<T: PartialOrd>(iter: impl Iterator<Item = T>) -> usize {
    let mut prev = None;
    let mut total = 0;
    for x in iter {
        if matches!(prev, Some(p) if p < x) {
            total += 1;
        }
        prev = Some(x);
    }
    total
}

fn main() {
    let nums: Vec<u32> = parse_lines("inputs/2021/p1.txt");
    let p1 = count_increasing(nums.iter());
    dbg!(p1);
    let p2 = count_increasing(nums.windows(3).map(|x| x.iter().sum::<u32>()));
    dbg!(p2);
}
