#![feature(let_chains)]

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Space {
    Vacant,
    Occupied(u8),
}

impl Space {
    fn to_char(&self) -> char {
        match self {
            Space::Vacant => '.',
            // todo better
            Space::Occupied(c) => match c {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                3 => 'D',
                _ => unimplemented!(),
            },
        }
    }
}

// room format left to right, [ top, bottom ]
// goal: [0, 0], [1, 1], [2, 2], [3, 3]

type Rooms = [[Space; 2]; 4];
type Hallway = [Space; 11];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct World {
    hallway: Hallway,
    rooms: Rooms,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Move {
    world: World,
    cost: i32,
}

const COSTS: [i32; 4] = [1, 10, 100, 1000];

impl World {
    fn is_solved(&self) -> bool {
        self.rooms
            == [
                [Space::Occupied(0); 2],
                [Space::Occupied(1); 2],
                [Space::Occupied(2); 2],
                [Space::Occupied(3); 2],
            ]
    }

    fn print(&self) {
        println!("#############");
        print!("#");
        for x in self.hallway {
            print!("{}", x.to_char());
        }
        println!("#");
        print!("###");
        for i in 0..4 {
            print!("{}#", self.rooms[i][0].to_char());
        }
        println!("##");
        print!("  #");
        for i in 0..4 {
            print!("{}#", self.rooms[i][1].to_char());
        }
        println!("");
        println!("  #########");
    }

    /// number of steps to move hallway[i] into the correct slot, or None if it's not possible
    fn num_steps(&self, i: usize, target: u8) -> Option<u8> {
        // check clogged
        let mut num_steps = 3;
        let bot = self.rooms[target as usize][1];
        let top = self.rooms[target as usize][0];
        if top != Space::Vacant {
            return None;
        }
        if let Space::Occupied(x) = bot {
            if x != target {
                return None;
            }
            num_steps -= 1; // otherwise one fewer step because don't have to go to bottom
        }
        // check hallway
        let mut from = i;
        let mut to = 2 * (1 + target) as usize;
        if from > to {
            std::mem::swap(&mut from, &mut to);
        }
        for check in from + 1..to {
            if matches!(self.hallway[check], Space::Occupied(_)) {
                return None;
            }
            num_steps += 1;
        }
        Some(num_steps)
    }

    fn valid_next_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        // First, attempt to move out of hallway. Force this move if exists.
        for i in 0..11 {
            if let Space::Occupied(target) = self.hallway[i] {
                if let Some(steps) = self.num_steps(i, target) {
                    let mut new_world = self.clone();
                    new_world.hallway[i] = Space::Vacant;
                    if new_world.rooms[target as usize][1] == Space::Vacant {
                        new_world.rooms[target as usize][1] = Space::Occupied(target);
                    } else {
                        new_world.rooms[target as usize][0] = Space::Occupied(target);
                    }
                    let cost: i32 = steps as i32 * COSTS[target as usize];
                    moves.push(Move {
                        world: new_world,
                        cost,
                    });
                    return moves;
                }
            }
        }
        // then generate all moves by entering every pod into the hallway at every valid slot
        for i in 0..4 {
            let bot = self.rooms[i][1];
            let top = self.rooms[i][0];
            if bot == Space::Vacant {
                // no one to move
                continue;
            }
            if bot == Space::Occupied(i as u8) && top == Space::Occupied(i as u8) {
                // solved
                continue;
            }
            if bot == Space::Occupied(i as u8) && top == Space::Vacant {
                // partial solve
                continue;
            }
            let mut base_steps = 2; // one to go up into top hall space plus one for initial
            let mover;
            if let Space::Occupied(_) = top {
                mover = 0;
            } else if let Space::Occupied(_) = bot {
                mover = 1;
                base_steps += 1;
            } else {
                unreachable!();
            }
            for direction in [-1, 1] {
                let mut cur_index = 2 * (i as i32 + 1) + direction;
                let mut cur_steps = base_steps;
                while cur_index >= 0 && cur_index < 11 && let Space::Vacant = self.hallway[cur_index as usize] {
                    let mut new_world = self.clone();
                    if let pod @ Space::Occupied(color) = new_world.rooms[i][mover] {
                        new_world.rooms[i][mover] = Space::Vacant;
                        new_world.hallway[cur_index as usize] = pod;
                        let cost = cur_steps * COSTS[color as usize];
                        moves.push(Move {
                            world: new_world,
                            cost
                        });
                    } else {
                        unreachable!();
                    }
                    if cur_index >= 3 && cur_index <= 7 {
                        cur_index += 2 * direction;
                        cur_steps += 2;
                    } else {
                        cur_index += direction;
                        cur_steps += 1;
                    }
                }
            }
        }
        // otherwise we have to move into the hallway - just try all
        moves
    }
}

// Test room and main room insert in middle:
// top
//   BCBD
// * DCBA
// * DBAC
//   ADCA
// bottom
// test problem solution 44169

fn main() {
    // test problem
    let hallway = [Space::Vacant; 11];
    // let rooms = [
    //     // top left       bottom left
    //     [Space::Occupied(1), Space::Occupied(0)],
    //     [Space::Occupied(2), Space::Occupied(3)],
    //     [Space::Occupied(1), Space::Occupied(2)],
    //     [Space::Occupied(3), Space::Occupied(0)],
    //     // top right      bottom right
    // ];
    // real
    let rooms = [
        // top left       bottom left
        [Space::Occupied(1), Space::Occupied(2)],
        [Space::Occupied(1), Space::Occupied(0)],
        [Space::Occupied(3), Space::Occupied(0)],
        [Space::Occupied(3), Space::Occupied(2)],
        // top right      bottom right
    ];

    let world = World { hallway, rooms };
    world.print();
    let mut queue = VecDeque::new();
    queue.push_back((world, 0));
    let mut cur_best = i32::MAX;
    while !queue.is_empty() {
        // DFS so we can prune effectively
        let (cand, cur_cost) = queue.pop_back().unwrap();
        for poss in cand.valid_next_moves() {
            let cost = cur_cost + poss.cost;
            if cost < cur_best {
                // only consider improvements to current best score
                if poss.world.is_solved() {
                    // println!("Solve {cost}");
                    cur_best = cost;
                } else {
                    queue.push_back((poss.world, cost));
                }
            }
        }
    }
    println!("{}", cur_best);
}
