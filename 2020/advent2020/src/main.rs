use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    advent2020::execute(&args[1]);
}
