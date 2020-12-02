use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub fn readfile(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(filename)?;
    let r = BufReader::new(f);
    return r.lines().collect();
}