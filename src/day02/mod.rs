use crate::io;

pub fn solve(test: bool) { 
    let filename = if test {"day02\\test.txt"} else {"day02\\input.txt"};
    
    let v: Vec<String> = io::readfile(filename).expect("File read failed.");

    // Answer to first puzzle
    let mut sum = 0;

    // Answer to second puzzle
    let mut sum2 = 0;

    for line in &v {
        // Go through the chars, find the indices of the dash separating the upper and lower bounds,
        // the first space, i.e. the one separating the bounds and the character, and the colon
        let mut dash_index: usize = 0;
        let mut space_index: usize = 0;
        let mut colon_index: usize = 0;

        for (i, c) in line.chars().enumerate() {
            match c {
                '-' => dash_index = i,                
                ':' => colon_index = i,
                ' ' => 
                if space_index == 0 {
                    space_index = i
                }
                _ => {}
            }
        }


        let lower: u8 = line[0 .. dash_index].parse().unwrap();
        let upper: u8 = line[(dash_index + 1) .. space_index].parse().unwrap();
        let c: char = line[(space_index + 1) .. colon_index].chars().next().unwrap();

        if valid(lower, upper, c, &line[colon_index + 2..]) {
            sum += 1;
        }
        if valid2(lower, upper, c, &line[colon_index + 2..]) {
            sum2 += 1;
        }
    }
    println!("Sum of valid passwords according to the first policy is {}", sum);
    println!("Sum of valid passwords according to the second policy is {}", sum2);
}

fn valid(lower: u8, upper: u8, c: char, line: &str) -> bool {
    let mut amt: u8 = 0;
    
    for ch in line.chars() {
        if ch == c {
            amt += 1;
        }
    }
    return amt >= lower && amt <= upper;
}

fn valid2(lower: u8, upper: u8, c: char, line: &str) -> bool {    
    let cs: Vec<char> = line.chars().collect();
    
    if cs[lower as usize - 1] == c {
        return cs[upper as usize -1] != c
    } else {
        return cs[upper as usize -1] == c
    }
}