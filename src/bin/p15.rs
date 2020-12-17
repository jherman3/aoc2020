const INPUT: &[usize] = &[12,1,16,3,11,0];
// const INPUT: &[usize] = &[0, 3, 6];

fn main() {
    let mut turns: Vec<_> = INPUT.into();

    for turn in INPUT.len()..2020 {
        let prev = turns[turn-1];
        if let Some(x) = turns[0..turn-1].iter().rposition(|&c| c==prev ) {
            turns.push(turn - 1 - x);
        } else {
            turns.push(0);
        }
        println!("{}: {}", turn, turns[turns.len()-1]);
    }
    println!("p1: {}", turns.last().unwrap());
}
