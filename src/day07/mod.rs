use crate::io;
use std::time::Instant;
use std::collections::HashMap;

struct Bag<'a> {
    name: &'a str,
    goldbags: Option<u32>,
    children: Vec<(usize, u32)>
}

impl<'a> Bag<'a> {
    fn countgolds(&self, bags: &Vec<Bag>) -> u32 {
        let mut sum: u32 = 0;
        for (c, n) in &self.children {
            if bags[*c].name == "shiny gold" {
                sum += n
            }
            sum += n * bags[*c].countgolds(bags);
        }
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

    fn getgolds(&mut self, bags: &Vec<Bag>) -> u32 {
        if self.goldbags.is_none() {
            self.goldbags = Some(self.countgolds(bags));
        }
        return self.goldbags.unwrap();
    }
}


pub fn solve(test: bool) -> u128 {    
    let filename = if test {"day07\\test.txt"} else {"day07\\input.txt"};

    let v: Vec<String> = io::readfile(filename).expect("File read failed.");
    
    println!("--- Day 07 ---");

    let time1 = Instant::now();

    let mut bags: Vec<Bag> = Vec::new();
    let mut map: HashMap<&str, usize> = HashMap::new();

    

    for line in &v {
        let mut spaces: Vec<usize> = Vec::new();

        for (i, c) in line.char_indices() {
            if c == ' ' {
                spaces.push(i);
            }
        }

        if spaces.len() == 6 {
            // Empty bag, move on
            continue;
        }

        let mainbag = &line[0..spaces[1]];

        // How many different bag types this bag can hold
        let numbags = (spaces.len() - 3) / 4;

        let mut i = 3;

        for _ in 0..numbags {
            let c_num = line[(spaces[i] + 1) .. spaces[i + 1]].parse::<u32>().unwrap();
            let c_bag = &line[(spaces[i + 1] + 1) .. spaces[i + 3]];

            if !map.contains_key(&mainbag) {
                bags.push(Bag{name: &mainbag, goldbags: None, children: Vec::new()});
                map.insert(&mainbag, bags.len() - 1);
            }

            if !map.contains_key(&c_bag) {
                bags.push(Bag{name: &c_bag, goldbags: None, children: Vec::new()});
                map.insert(&c_bag, bags.len() - 1);
            }

            bags[*map.get(&mainbag).unwrap()].children.push((*map.get(&c_bag).unwrap(), c_num));


            i += 4;
        }
    }

    let mut sum = 0;
    for bag in &bags {
        if bag.countgolds(&bags) > 0{
            sum += 1;
        }
    }

    let sum2 = bags[*map.get("shiny gold").unwrap()].countchildren(&bags);

    let timetaken = time1.elapsed().as_micros();
    
    println!("Solution to part one: {}", sum);
    println!("Solution to part two: {}", sum2);

    println!("Time elapsed: {} µs", timetaken);

    return timetaken;
}