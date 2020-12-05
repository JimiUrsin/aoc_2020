use crate::io;
use std::collections::HashSet;
use std::time::Instant;

pub fn solve(test: bool) {
    let filename = if test {"day05\\test.txt"} else {"day05\\input.txt"};

    let v: Vec<String> = io::readfile(filename).expect("File read failed.");

    let time1 = Instant::now();

    let mut max_id = 0;

    // Used for knowing when to stop iteration in part two
    let mut max_row = 0;

    let mut set: HashSet<(i16, i16)> = HashSet::new();

    for line in &v {
        // We actually only need the lower bound for the row and seat,
        // the upper and lower bound will end up being the same when
        // we find the row and seat numbers we're looking for.
        let mut row = 0;
        let mut r_d = 64;

        let mut seat = 0;
        let mut s_d = 4;

        for c in line.chars() {
            match c {
                'F' => {
                    r_d /= 2;
                },
                'B' => {
                    row += r_d;
                    r_d /= 2;
                }
                'L' => {
                    s_d /= 2;
                }
                'R' => {
                    seat += s_d;
                    s_d /= 2;
                }
                _ => {} // Should never be hit
            }
        }

        max_id = Ord::max(max_id, row * 8 + seat);
        max_row = Ord::max(max_row, row);
        set.insert((row, seat));
    }
    println!("Solution to part one: {}", max_id);
    println!("Time elapsed: {} µs\n", time1.elapsed().as_micros());

    let time2 = Instant::now();

    // Go through all rows and seats and check if they're missing
    
    // If there is only one seat missing from the row and it's not
    // the first or last one, that's our solution for part two 
    for i in 0..max_row {
        let mut ms = (-1, -1);      
        let mut num = 0;

        for j in 0..=7 {
            if !set.contains(&(i, j)) {
                ms = (i, j);
                num += 1;
            }
        }
        if num == 1 && ms.1 > 0 && ms.1 < 7{
            println!("Solution to part two: {}", ms.0 * 8 + ms.1);
        }
    }
    
    println!("Time elapsed: {} µs\n", time2.elapsed().as_micros());
}