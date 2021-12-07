use anyhow::Result;
use aoc2020::each_line;

#[derive(Copy, Clone, Debug)]
enum Criteria {
    MostCommon,
    LeastCommon,
}

fn count_bits_rev(items: &[String]) -> Vec<usize> {
    let mut bitcounts = vec![0; items[0].len()];
    for x in items {
        for (i, b) in x.chars().rev().enumerate() {
            if b == '1' {
                bitcounts[i] += 1;
            }
        }
    }
    bitcounts
}

fn most_common_p1(items: &[String], criteria: Criteria) -> String {
    count_bits_rev(items)
        .iter()
        .map(|&x| {
            let double = 2 * x;
            match criteria {
                Criteria::MostCommon => {
                    if double >= items.len() {
                        '1'
                    } else {
                        '0'
                    }
                }
                Criteria::LeastCommon => {
                    if double >= items.len() {
                        // one is most common - take 0
                        '0'
                    } else {
                        '1'
                    }
                }
            }
        })
        .rev()
        .collect()
}

// lots of wasted perf because of calcing entire string - oh well
fn part2(bits: &[String], criteria: Criteria) -> String {
    let mut cands = bits.to_vec();
    let mut i = 0;
    while cands.len() > 1 && i < cands[0].len() {
        let counts = most_common_p1(&cands, criteria);
        cands = cands
            .into_iter()
            .filter(|x| x[i..=i] == counts[i..=i])
            .collect();
        i += 1;
    }
    let x = cands.drain(0..=0).next().expect("empty cand");
    x
}

fn main() -> Result<()> {
    let bits: Vec<_> = each_line("inputs/2021/p3.txt").collect();
    let bitstr = most_common_p1(&bits, Criteria::MostCommon);
    let num = u32::from_str_radix(&bitstr, 2)?;
    let mut mask = 0;
    for i in 0..bits[0].len() {
        mask |= 1 << i;
    }
    let p1 = num * ((!num) & mask);
    dbg!(p1);

    let oxy = part2(&bits, Criteria::MostCommon);
    let co2 = part2(&bits, Criteria::LeastCommon);
    let p2 = u32::from_str_radix(&oxy, 2)? * u32::from_str_radix(&co2, 2)?;
    dbg!(p2);
    Ok(())
}
