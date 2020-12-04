// fn main() {
//     let input = 36000000;
//     let target = input / 10;
//     let mut houses = vec![0; target];
//     for i in 1..target {
//         let mut mult = 1;
//         while i * mult < target {
//             houses[i * mult] += i;
//             mult += 1;
//         }
//     }
//     for i in 0..houses.len() {
//         if houses[i] >= target {
//             dbg!(i);
//             break;
//         }
//     }
// }

fn main() {
    let input = 36000000;
    let target = input / 11;
    let mut houses = vec![0; target];
    for i in 1..target {
        let mut mult = 1;
        while i * mult < target && mult < 51 {
            houses[i * mult] += i;
            mult += 1;
        }
    }
    for i in 0..houses.len() {
        if houses[i] >= target {
            dbg!(i);
            break;
        }
    }
}
