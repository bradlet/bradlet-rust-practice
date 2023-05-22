/// program_my_macro.rs
/// @author bradlet
use text_colorizer::Colorize;



pub fn main(args: Vec<String>) -> Vec<String> {
    println!("{}", "Welcome to My Macro Demo!".bright_cyan());

    args
}
