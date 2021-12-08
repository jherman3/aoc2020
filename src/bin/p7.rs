fn main() {
    let mut input: Vec<i32> = aoc2020::read_file("inputs/2021/p7.txt")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    input.sort();
    let median = input[input.len() / 2];
    let cost = |t: i32| input.iter().map(|x| (x - t).abs()).sum::<i32>();
    let p1 = cost(median);
    // tricky so just bruteforce - it's fast
    let min = input[0];
    let max = input[input.len() - 1];
    let cost = |t: i32| {
        input
            .iter()
            .map(|x| {
                let n = (x - t).abs();
                n * (n + 1) / 2
            })
            .sum::<i32>()
    };
    let p2 = (min..=max).map(cost).min().unwrap();
    dbg!(p1, p2);
}
