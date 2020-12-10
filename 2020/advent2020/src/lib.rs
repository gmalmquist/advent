pub mod day01;

pub fn execute(day: &str) {
    println!("Executing code for day {}", day);
    match day {
        "01" => day01::main(),
        "02" => day02::main(),
        _ => eprintln!("Unknown day: {}", day),
    }
}
