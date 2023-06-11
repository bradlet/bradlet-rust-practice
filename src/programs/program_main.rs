/// program_main.rs
/// @author bradlet
use text_colorizer::Colorize;

pub fn main(args: Vec<String>) -> Vec<String> {
    let welcome = "Welcome to Main!".bright_cyan();
    println!("{}", welcome);

    args
}
