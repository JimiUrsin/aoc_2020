use crate::io;
use std::time::Instant;

pub fn solve(test: bool) -> u128 {    
    let filename = if test {"day06\\test.txt"} else {"day06\\input.txt"};

    let v: Vec<String> = io::readfile(filename).expect("File read failed.");
    
    println!("--- Day 06 ---");

    let time1 = Instant::now();

    let mut x: [u16; 26] = [0; 26];

    let mut sum = 0;
    let mut sum2 = 0;

    let mut people = 0;

    for line in &v {
        if line.is_empty() {
            // Go through all 26 letter counters
            for n in x.iter() {
                // If it's not 0 (one or more people from the group said "yes" 
                // to this question) increment the solution to part one.
                if n != &0 {
                    sum += 1;
                }

                // If every person in the group answered yes to this question,
                // increment the solution for part two.
                if n == &people {
                    sum2 += 1;
                }
            }

            // Reset counters for the next group
            people = 0;
            x = [0; 26];

            continue;
        }

        // Turn the lower case ASCII character to an index and increment it
        // Ugly but works :^)
        for c in line.chars() {
            x[c as usize - 97] += 1;
        }

        people += 1;
    }

    println!("Solution to part one: {}", sum);
    println!("Solution to part two: {}", sum2);

    let timetaken = time1.elapsed().as_micros();
    println!("Time elapsed: {} µs", timetaken);
    return timetaken;
}