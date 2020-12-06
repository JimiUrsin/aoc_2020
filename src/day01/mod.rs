use crate::io;
use std::collections::HashSet;
use std::time::Instant;

pub fn solve(test: bool) -> u128 {
    let filename = if test {"day01\\test.txt"} else {"day01\\input.txt"};

    let lines: Vec<String> = io::readfile(filename).expect("How did reading the file fail, what are you doing?");

    println!("--- Day 01 ---");

    let t1 = Instant::now();
    let mut s: HashSet<i32> = HashSet::new();
    let mut v: Vec<i32> = Vec::new();

    let num = 2020;
    let mut found = false;

    for line in &lines {
        let li: i32 = line.parse().unwrap();

        // Part one
        if !found && s.contains(&(num - li)) {
            println!("Solution to part one: {}", (num - li) * li);
            found = true;
        } else {
            s.insert(li);
        }

        // Add parsed integer to vector for part two
        v.push(li);
    }

    // Part two

    /*
    // Naive cubic method
    let t2 = Instant::now();
    'outerestest:for i in 0 .. (v.len() - 2) {
        for j in (i + 1) .. (v.len() - 1) {
            for k in (j + 1) .. v.len() {
                if v[i] + v[j] + v[k] == num {
                    println!("Naive method found a match, {}, {} and {}", v[i], v[j], v[k]);
                    println!("The product of these numbers is {}", v[i] as i64 * v[j] as i64 * v[k] as i64);
                    break 'outerestest;
                }
            }
        }
    }

    println!("Time elapsed: {} ms", t2.elapsed().as_millis());
    println!();

    // Alternative vectorized method, this was faster than a set
    let t3 = Instant::now();

    let mut tuples: Vec<(i32, i32, i32)> = Vec::new();
    for i in 0..v.len() - 1{
        for j in 1..v.len() {
            tuples.push((v[i], v[j], v[i] + v[j]));
        }
    }

    'outerer:for i in 0 .. v.len() {
        for t in &tuples {
            if v[i] + t.2 == num {                
                println!("Vectorized method found a match for part two, {}, {} and {}", v[i], t.0, t.1);
                println!("The product of these numbers is {}", v[i] * t.0 * t.1);
                break 'outerer;
            }
        }
    }
    println!("Time elapsed: {} ms", t3.elapsed().as_millis());
    println!();

    */

    // Sorting method, this just dumped on the other two
    v.sort();
    let len = v.len() - 1;
    'o3:for (i, n) in v.iter().enumerate() {
        let mut j = i + 1;
        let mut k = len;

        while j != k {
            let sum = n + v[j] + v[k];

            if sum == num {
                println!("Solution to part two: {}", n * v[j] * v[k]);
                break 'o3;
            }
            if sum > num {
                k -= 1;
            } else if sum < num {
                j += 1;
            }
        }
    }
    
    let timetaken = t1.elapsed().as_micros();
    println!("Time elapsed: {} µs", timetaken);
    return timetaken;
}