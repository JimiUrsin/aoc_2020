use crate::io;
use std::time::Instant;

pub fn solve(test: bool) {
    let filename = if test {"day04\\test.txt"} else {"day04\\input.txt"};

    let v: Vec<String> = io::readfile(filename).expect("File read failed.");

    let time1 = Instant::now();

    let mut flags: u8 = 0;
    let mut flags2: u8 = 0;

    let mut sum = 0;
    let mut sum2 = 0;

    for line in &v {
        if line.is_empty() {
            // Blank line found, increment sum if all required flags are set, sum2 if all are valid too
            if flags == 0b01111111 {
                sum += 1;
            }
            if flags2 == 0b01111111 {
                sum2 += 1;
            }
            flags = 0;
            flags2 = 0;
            continue;
        }

        // Split into key:value pairs
        for field in line.split(' ') {
            // The value (anything after the colon)
            let val = &field[4 ..];

            // Match the three letter key, set a flag for part one if valid key was found
            // also checks the value and sets a flag for part two if it was valid
            match &field[.. 3] {
                "byr" => {
                    flags |= 0b01000000;
                    if num_valid(val, 1920, 2002) {       
                        flags2 |= 0b01000000;
                    }
                }
                "iyr" => {
                    flags |= 0b00100000;
                    if num_valid(val, 2010, 2020) {
                        flags2 |= 0b00100000;
                    }
                },
                "eyr" => {
                    flags |= 0b00010000;
                    if num_valid(val, 2020, 2030) {
                        flags2 |= 0b00010000;
                    }
                },
                "hgt" => {
                    flags |= 0b00001000;
                    if hgt_valid(val) {
                        flags2 |= 0b00001000;
                    }
                },
                "hcl" => {
                    flags |= 0b00000100;
                    if hcl_valid(val) {
                        flags2 |= 0b00000100;
                    }
                },
                "ecl" => {
                    flags |= 0b00000010;
                    if ecl_valid(val) {
                        flags2 |= 0b00000010;
                    }
                },
                "pid" => {
                    flags |= 0b00000001;
                    if pid_valid(val) {
                        flags2 |= 0b00000001;
                    }
                },
                // Key was invalid, ignore the key:value pair
                _ => {}
            }
        }

    }

    println!("Answer to part one: {}", sum);
    println!("Answer to part two: {}", sum2);
    println!("Time elapsed: {} µs", time1.elapsed().as_micros());
}

fn num_valid(s_num: &str, lower: u16, upper: u16) -> bool {
    let num: u16 = match s_num.parse() {
        Ok(i) => i,
        Err(_) => return false
    };
    return num >= lower && num <= upper;
}

fn hgt_valid(s_hgt: &str) -> bool {
    let l = match &s_hgt[s_hgt.len() - 2 ..] {
        "cm" => 3,
        "in" => 2,
        _ => return false
    };

    // Check that height matches allowed range for given unit
    if l == 3 {
        return num_valid(&s_hgt[.. l], 150, 193);
    } else {
        return num_valid(&s_hgt[.. l], 59, 76);
    }
}

fn hcl_valid(s_hcl: &str) -> bool {
    let mut chars = s_hcl.chars();

    if chars.next().unwrap() != '#' {
        return false;
    }

    let mut sum = 0;
    for c in chars {
        // Check if digit is 0-9 or a-f (A-F not allowed (I think?))
        if c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_lowercase())  {
            sum += 1;
        } else {
            return false;
        }
    }

    return sum == 6;
}

fn ecl_valid(s_ecl: &str) -> bool {
    return match s_ecl {
        "amb" |
        "blu" | 
        "brn" |
        "gry" |
        "grn" |
        "hzl" |
        "oth" => true,
        _ => false
    }
}

fn pid_valid(s_pid: &str) -> bool {
    let mut sum = 0;

    for c in s_pid.chars() {
        if !c.is_ascii_digit() {
            return false;
        }

        sum += 1;
    }

    return sum == 9;
}