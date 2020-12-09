#![feature(str_split_once)]
use aoc2020::each_line;
use std::collections::HashSet;

enum Inst {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
enum Res {
    Loop(i32),
    Done(i32),
    Invalid,
}

fn main() {
    let mut insts: Vec<Inst> = Vec::new();
    for l in each_line("inputs/p8.txt") {
        let (op, num) = l.split_once(" ").unwrap();
        let x = num.parse().unwrap();
        insts.push(match op {
            "nop" => Inst::Nop(x),
            "jmp" => Inst::Jmp(x),
            "acc" => Inst::Acc(x),
            _ => unimplemented!(),
        });
    }
    println!("Part 1: {:?}", execute(&insts));

    for i in 0..insts.len() {
        if let Inst::Jmp(x) = insts[i] {
            insts[i] = Inst::Nop(x);
            if let Res::Done(res) = execute(&insts) {
                println!("Part 2: {}", res);
                break;
            }
            insts[i] = Inst::Jmp(x);
        }
        if let Inst::Nop(x) = insts[i] {
            insts[i] = Inst::Jmp(x);
            if let Res::Done(res) = execute(&insts) {
                println!("Part 2: {}", res);
                break;
            }
            insts[i] = Inst::Nop(x);
        }
    }
}

fn execute(insts: &[Inst]) -> Res {
    let mut ip: i32 = 0;
    let mut acc: i32 = 0;
    let mut idx = HashSet::new();
    while (ip as usize) < insts.len() {
        if idx.contains(&ip) {
            return Res::Loop(acc);
        }
        idx.insert(ip);
        match insts[ip as usize] {
            Inst::Nop(_) => {}
            Inst::Acc(i) => acc += i,
            Inst::Jmp(i) => ip += i - 1,
        }
        ip += 1;
    }
    if ip as usize == insts.len() {
        Res::Done(acc)
    } else {
        Res::Invalid
    }
}
