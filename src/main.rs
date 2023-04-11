/// PracticeRust
/// @author bradlet
/// A project that I will use to keep track of my practice programs as I acquaint myself with Rust.
mod models;
mod programs;

use crate::programs::Program;
use bradleys_random_rust_helpers::*;
use text_colorizer::Color;

fn main() {
    print!("Welcome to program selection!\nYour choices... ");
    for program in Program::programs() {
        print!("{:?}, ", program)
    }
    println!(); horizontal_sep(40, Some(Color::BrightGreen));

    let args: Vec<String> = std::env::args().collect();

    let program_selection = args.get(1).expect("No CLI input provided.");

    Program::run(String::from(program_selection), args)
}
