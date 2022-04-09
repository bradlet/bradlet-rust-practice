/*
    ### PracticeRust
    ##### Bradley Thompson
    A project that I will use to keep track of my practice programs as I acquaint myself with Rust.
 */

use std::env::Args;

fn main() {
    let mut args: Args = std::env::args();
    if args.len() != 2 { panic!("Invalid arguments provided") }

    // Note to self: args.nth(n) is destructive and drops all parsed elements in the iterator
    //  Also: 0th element is the name of this program.
    let input = args.nth(1).expect("No CLI input provided.");
    println!("Hello, world!\nArg 1: {}\n", input);
}
