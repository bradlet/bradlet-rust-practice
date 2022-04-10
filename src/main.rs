/// PracticeRust
/// @author bradlet
/// A project that I will use to keep track of my practice programs as I acquaint myself with Rust.
mod models;
mod helpers;

use crate::models::types::Program;

fn main() {
    print!("Welcome to program selection!\nYour choices... ");
    for program in Program::programs() {
        print!("{:?}, ", program)
    }
    println!(); helpers::horizontal_sep();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 { panic!("Invalid arguments provided") }

    // Note to self: args.nth(n) is destructive and drops all parsed elements in the iterator
    //  Also: 0th element is the name of this program.
    let input = args.get(1).expect("No CLI input provided.");

    Program::run(String::from(input), args)
}
