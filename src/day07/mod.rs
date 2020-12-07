use crate::io;
use std::time::Instant;
use std::collections::HashMap;

struct Bag<'a> {
    name: &'a str,
    children: Vec<(usize, u32)>
}

impl<'a> Bag<'a> {
    // Totally thought we'd have to calculate the amount of gold bags contained
    // in some of the bags in part two. Oh well
    fn countgolds(&self, bags: &Vec<Bag>, golds: &mut Vec<Option<u32>>, idx: &usize) -> u32 {
        if golds[*idx].is_some() {
            return golds[*idx].unwrap();
        }
        let mut sum: u32 = 0;
        for (c, n) in &self.children {
            if bags[*c].name == "shiny gold" {
                sum += n;
                break;
            }
            sum += n * bags[*c].countgolds(bags, golds, c);
        }
        golds[*idx] = Some(sum);
        return sum;
    }

    fn countchildren(&self, bags: &Vec<Bag>) -> u32 {
        let mut sum: u32 = 1;
        if self.children.is_empty() {
            return sum;
        }
        for (c, i) in &self.children {
            sum += bags[*c].countchildren(&bags) * i;
        }
        return sum;
    }
}

pub fn solve(test: bool) -> u128 {    
    let filename = if test {"day07\\test.txt"} else {"day07\\input.txt"};

    let v: Vec<String> = io::readfile(filename).expect("File read failed.");
    
    println!("--- Day 07 ---");

    let time1 = Instant::now();

    let mut bags: Vec<Bag> = Vec::new();

    // Contains the (name, index) pairings, bag names into indices for the
    // bags vector above
    let mut map: HashMap<&str, usize> = HashMap::new();
  

    for line in &v {

        let mut spaces: Vec<usize> = Vec::new();

        // Find the indices of all spaces in the line, these amount of spaces will
        // be used to determine how many different bag types this bag can hold
        for (i, c) in line.char_indices() {
            if c == ' ' {
                spaces.push(i);
            }
        }

        if spaces.len() == 6 {
            // Empty bag, move on
            continue;
        }

        // Bag whose contents are shown on the current line.
        let mainbag = &line[0..spaces[1]];

        // How many different bag types this bag can hold
        let numbags = (spaces.len() - 3) / 4;

        // Start from the fourth space, so after the word "contain"
        let mut i = 3;

        for _ in 0..numbags {
            // Get the number and type of bags that will fit into the bag mentioned
            // at the start of the current line
            let c_num = line[(spaces[i] + 1) .. spaces[i + 1]].parse::<u32>().unwrap();
            let c_bag = &line[(spaces[i + 1] + 1) .. spaces[i + 3]];

            // If we haven't yet encountered the main bag, add it to the vector and
            // insert its index into the map of (bag name : vector index) pairings
            if !map.contains_key(&mainbag) {
                bags.push(Bag{name: &mainbag, children: Vec::new()});
                map.insert(&mainbag, bags.len() - 1);
            }

            // Do the same for the bag contained in the main bag
            if !map.contains_key(&c_bag) {
                bags.push(Bag{name: &c_bag, children: Vec::new()});
                map.insert(&c_bag, bags.len() - 1);
            }

            // Beautiful
            bags[*map.get(&mainbag).unwrap()].children.push((*map.get(&c_bag).unwrap(), c_num));

            // Move 4 spaces forward, to the start of the next bag
            i += 4;
        }
    }

    let mut sum = 0;


    let mut golds: Vec<Option<u32>> = vec![None; bags.len()];

    for bag in &bags {
        let idx = map.get(&bag.name).unwrap();
        bag.countgolds(&bags, &mut golds, idx);
        if golds[*idx].unwrap() > 0 {
            sum += 1;
        }
    }

    let sum2 = bags[*map.get("shiny gold").unwrap()].countchildren(&bags) - 1; // Remove the gold bag itself from the sum

    let timetaken = time1.elapsed().as_micros();
    
    println!("Solution to part one: {}", sum);
    println!("Solution to part two: {}", sum2);

    println!("Time elapsed: {} µs", timetaken);

    return timetaken;
}