use aoc2020::each_line;
use ring_algorithm::chinese_remainder_theorem;

fn main() {
    let ls: Vec<_> = each_line("inputs/p13.txt").collect();
    let depart: i32 = ls[0].parse().unwrap();
    let busses: Vec<i32> = ls[1]
        .split(",")
        .filter(|&c| c != "x")
        .map(|c| c.parse().unwrap())
        .collect();
    // time you beat bus = depart % b
    // wait time = (-depart) mod b
    let ts = busses
        .iter()
        .map(|&b| (b, (-depart).rem_euclid(b)))
        .min_by_key(|t| t.1)
        .unwrap();
    let p1 = ts.0 * ts.1;
    dbg!(p1);

    // Chinese remainder theorem
    // Solving system t = an mod bn
    // each an is same trick as above
    let (an, bn): (Vec<_>, Vec<_>) = ls[1]
        .split(",")
        .enumerate()
        .filter(|&(_, c)| c != "x")
        .map(|(i, c)| (i as i64, c.parse::<i64>().unwrap()))
        .map(|(i, bn)| ((-i).rem_euclid(bn), bn))
        .unzip();
    let t = chinese_remainder_theorem(&an, &bn).unwrap();
    let p2 = t.rem_euclid(bn.iter().product());
    dbg!(p2);
}
