pub mod io;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let day = 4;
    let test = false;

    println!("--- Day {:02} --", day);
    match day {
        1 => day01::solve(test),
        2 => day02::solve(test),
        3 => day03::solve(test),
        4 => day04::solve(test),
        5 => day05::solve(test),
        _ => {}
    }
}

