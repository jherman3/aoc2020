use aoc2020::{each_line, AnyResult};
use std::collections::{HashMap, HashSet};

fn main() -> AnyResult<()> {
    let mut adjacency = HashMap::new();
    for pair in each_line("inputs/2021/p12.txt") {
        let (p1, p2) = pair.split_once("-").expect("split");
        // todo map to ints instead of allocating everywhere?
        adjacency
            .entry(p1.to_owned())
            .or_insert_with(HashSet::new)
            .insert(p2.to_owned());
        adjacency
            .entry(p2.to_owned())
            .or_insert_with(HashSet::new)
            .insert(p1.to_owned());
    }
    //dbg!(adjacency);
    // DFS on input
    let mut visited = HashMap::new();
    visited.insert("start", 1);
    let p1 = num_paths(&adjacency, "start", "end", &mut visited, false);
    dbg!(p1);
    let p2 = num_paths(&adjacency, "start", "end", &mut visited, true);
    dbg!(p2);
    Ok(())
}

fn num_paths<'a>(
    adjacency: &'a HashMap<String, HashSet<String>>,
    start: &str,
    end: &str,
    visited: &mut HashMap<&'a str, usize>,
    part2: bool,
) -> usize {
    // number of paths from start to end that obey the rules
    let mut count = 0;
    for neighbor in adjacency.get(start).unwrap() {
        if neighbor == end {
            count += 1;
            continue;
        }
        let neighbor: &str = neighbor.as_ref();
        if neighbor.chars().all(char::is_lowercase) {
            #[allow(clippy::map_entry)]  // clunky because the later remove
            if !visited.contains_key(&neighbor) {
                visited.insert(neighbor, 1);
                count += num_paths(adjacency, neighbor, end, visited, part2);
                visited.remove(&neighbor);
            } else if part2 && neighbor != "start" && visited.values().all(|&x| x < 2) {
                // havent used the extra path, do so
                *visited.get_mut(&neighbor).unwrap() += 1;
                count += num_paths(adjacency, neighbor, end, visited, part2);
                *visited.get_mut(&neighbor).unwrap() -= 1;
            }
        } else {
            count += num_paths(adjacency, neighbor, end, visited, part2);
        }
    }
    count
}
