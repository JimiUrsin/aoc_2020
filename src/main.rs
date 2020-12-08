pub mod io;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let day = 0;
    let test = false;

    match day {
        0 => {runall();},
        1 => {day01::solve(test);},
        2 => {day02::solve(test);},
        3 => {day03::solve(test);},
        4 => {day04::solve(test);},
        5 => {day05::solve(test);},
        6 => {day06::solve(test);},
        7 => {day07::solve(test);},
        8 => {day08::solve(test);},
        _ => {}
    }
}

fn runall() -> u128 {
    let mut timetaken = 0;

    timetaken += day01::solve(false);
    println!();
    timetaken += day02::solve(false);
    println!();
    timetaken += day03::solve(false);
    println!();
    timetaken += day04::solve(false);
    println!();
    timetaken += day05::solve(false);
    println!();
    timetaken += day06::solve(false);
    println!();
    timetaken += day07::solve(false);
    println!();
    timetaken += day08::solve(false);
    println!();
    
    println!("Time taken to find all solutions was {:.2} ms", (timetaken as f64) / 1000.0);

    return timetaken;
}

