use crate::io;
use std::collections::HashSet;

pub fn solve() {
    let lines: Vec<String> = io::readfile("day01\\input.txt").expect("How did reading the file fail, what are you doing?");

    let mut s: HashSet<i32> = HashSet::new();
    let mut v: Vec<i32> = Vec::new();

    let num = 2020;
    let mut found = false;

    for line in &lines {
        let li: i32 = line.parse().unwrap();

        // Part one
        if !found && s.contains(&(num - li)) {
            println!("Found a match for part one, {} and {}", num - li, li);
            println!("The product of these numbers is {}", (num - li) * li);
            found = true;
        } else {
            s.insert(li);
        }

        // Add parsed integer to vector for part two
        v.push(li);
    }


    // Part two

    'outerestest:for i in 0..v.len() - 2 {
        for j in 1..v.len() - 1 {
            for k in 2..v.len() {
                if v[i] + v[j] + v[k] == num {
                    println!("Found a match, {}, {} and {} part two", v[i], v[j], v[k]);
                    println!("The product of these numbers is {}", v[i] as i64 * v[j] as i64 * v[k] as i64);
                    break 'outerestest;
                }
            }
        }
    }
}