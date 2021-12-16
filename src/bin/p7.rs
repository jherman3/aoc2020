fn main() {
    let mut input: Vec<i32> = aoc2020::read_file("inputs/2021/p7.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    input.sort_unstable();
    let median = input[input.len() / 2];
    let cost = |t: i32| input.iter().map(|x| (x - t).abs()).sum::<i32>();
    let p1 = cost(median);

    let mean = (input.iter().sum::<i32>() as f32 / input.len() as f32).round() as i32;
    let cost = |t: i32| {
        input
            .iter()
            .map(|x| {
                let n = (x - t).abs();
                n * (n + 1) / 2
            })
            .sum::<i32>()
    };
    let p2 = *[cost(mean), cost(mean + 1), cost(mean - 1)]
        .iter()
        .min()
        .unwrap();
    dbg!(p1, p2);
}
