/// program_displayable.rs
/// @author bradlet
use crate::models::displayable_object::DisplayableThing;

pub fn main(args: Vec<String>) -> Vec<String> {
    let displayable = DisplayableThing {
        first_value: args.get(2).expect("no arg").clone(),
        second_value: args.get(3).expect("no arg").clone()
    };

    println!("Here's your input: {}", displayable);

    // Learned via the following that tuples don't implement Display.
    // println!("Tuple practice: {}", tup);
    let tup: (&String, &String) = (&displayable.first_value, &displayable.second_value);
    let (first, second) = tup;
    // So, because references themselves are just memory addresses, they can be stored on the stack.
    // So, first and second get the reference copied, no ownership is moved from tup!

    println!("1st: {}; 2nd: {}", first, second);
    println!("from tup: {} & {}", tup.0, tup.1);

    args
}
