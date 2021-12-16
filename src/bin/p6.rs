use cached::proc_macro::cached;

#[cached]
fn count_fish(timer: i32, days: i32) -> usize {
    let mut timer = timer; // hack - cached macro doesnt support mut
    let mut days = days;
    let mut num_fish = 1;
    while days - timer > 0 {
        // 7 and 9 for timer values because fish don't appear until the next day.
        days -= timer;
        num_fish += count_fish(9, days);
        timer = 7;
    }
    num_fish
}

fn main() {
    let input: Vec<i32> = aoc2020::read_file("inputs/2021/p6.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut total_fish_p1 = 0;
    let mut total_fish_p2 = 0;
    for x in &input {
        total_fish_p1 += count_fish(*x, 80);
        total_fish_p2 += count_fish(*x, 256);
    }
    dbg!(total_fish_p1, total_fish_p2);
}
