use crate::io;
use std::time::{Instant};

pub fn solve() {
    let v: Vec<String> = io::readfile("day03\\input.txt").expect("File read failed.");
    let start1 = Instant::now();
    
    let ans = slide(&v, 3, 1);

    let mut prod = slide(&v, 1, 1);
    prod *= ans;
    prod *= slide(&v, 5, 1);
    prod *= slide(&v, 7, 1);
    prod *= slide(&v, 1, 2);

    println!("Answer to first part: {}", ans);
    println!("Answer to second part: {}", prod);
    println!("Time elapsed: {} µs", start1.elapsed().as_micros());
}

fn slide(v: &Vec<String>, dx: usize, dy: usize) -> i64 {
    let mut sum = 0;

    let mut y = 0;
    let mut x = 0;

    let div = v[0].len();

    while y < v.len() {
        if v[y].chars().nth(x).unwrap() == '#' {
            sum += 1;
        }
        y += dy;
        x += dx;
        x %= div;
    }

    return sum;
}