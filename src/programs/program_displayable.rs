/// program_displayable.rs
/// @author bradlet
use crate::models::displayable_object::DisplayableThing;

pub fn main(args: Vec<String>) -> Vec<String> {
    let displayable = DisplayableThing {
        first_value: args.get(2).expect("no arg").clone(),
        second_value: args.get(3).expect("no arg").clone()
    };

    println!("Here's your input: {}", displayable);

    args
}
