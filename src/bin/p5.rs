use aoc2020::each_line;

fn main() {
    let mut ids: Vec<_> = each_line("inputs/p5.txt").map(|l| get_seat_id(&l)).collect();
    dbg!(ids.iter().max().unwrap());
    ids.sort();
    let first = ids[0];
    for (i, x) in ids.iter().enumerate() {
        let seat = first + i as u32;
        if seat != *x {
            dbg!(seat);
            break;
        }
    }
}

fn get_seat_id(code: &str) -> u32 {
    let first: String = code.chars().take(7).map(|c| if c == 'B' { '1' } else { '0' } ).collect();
    let row = u32::from_str_radix(&first, 2).unwrap();
    let second: String = code.chars().skip(7).map(|c| if c == 'R' { '1' } else {'0'}).collect();
    let col = u32::from_str_radix(&second, 2).unwrap();
    row * 8 + col
}
