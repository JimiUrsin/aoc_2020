use crate::io;
use std::time::Instant;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Cmd {
    ACC,
    JMP,
    NOP
}

pub fn solve(test: bool) -> u128 {
    let filename = if test {"day08\\test.txt"} else {"day08\\input.txt"};

    let v: Vec<String> = io::readfile(filename).expect("File read failed.");
    
    println!("--- Day 08 ---");

    let time1 = Instant::now();

    let mut pc: i32 = 0;
    let mut acc = 0;

    let mut set: HashSet<i32> = HashSet::new();
    let mut v2: Vec<(Cmd, i32)> = Vec::new();

    for line in &v {
        match &line[..3] {
            "nop" => v2.push((Cmd::NOP, 0)),
            "acc" => v2.push((Cmd::ACC, line[4..].parse::<i32>().unwrap())),
            "jmp" => v2.push((Cmd::JMP, line[4..].parse::<i32>().unwrap())),
            _ => {}
        }
    }

    // Go through the program, adding the program counter to a set each time
    // if the set contains the current program counter, we've hit the same line
    // i.e. we've hit a loop.
    loop {
        if set.contains(&pc) {
            println!("Solution to part one: {}", acc);
            break;
        }
        set.insert(pc);

        let cmd = &v2[pc as usize];
        match &cmd.0 {
            Cmd::ACC => {acc += &cmd.1; pc += 1},
            Cmd::JMP => pc += &cmd.1,
            Cmd::NOP => pc += 1
        }
    }

    set.clear();

    let mut first = true;
    let mut lastone = 0;
    let len: i32 = v2.len() as i32;
    'o:loop {
        if !first {
            v2[lastone].0 = Cmd::JMP;
        }
        first = false;
        for i in lastone + 1..v2.len() {
            if v2[i].0 == Cmd::JMP {
                v2[i].0 = Cmd::NOP;
                lastone = i;
                break;
            }
        }
        pc = 0;
        acc = 0;
        loop {
            if pc >= len {
                println!("Solution to part two: {}", &acc);
                break 'o;
            }
            if set.contains(&pc) {
                set.clear();
                break;
            }
            set.insert(pc);

            let cmd = &v2[pc as usize];
            match &cmd.0 {
                Cmd::ACC => {acc += &cmd.1; pc += 1},
                Cmd::JMP => pc += &cmd.1,
                Cmd::NOP => pc += 1
            }
        }
    }

    let timetaken = time1.elapsed().as_micros();
    println!("Time elapsed: {} µs", &timetaken);
    return timetaken;
}