pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub fn execute(day: &str) {
    println!("Executing code for day {}", day);
    match day {
        "01" => day01::main(),
        "02" => day02::main(),
        "03" => day03::main(),
        "04" => day04::main(),
        "05" => day05::main(),
        "06" => day06::main(),
        "07" => day07::main(),
        "08" => day08::main(),
        "09" => day09::main(),
        "10" => day10::main(),
        "11" => day11::main(),
        _ => eprintln!("Unknown day: {}", day),
    }
}
