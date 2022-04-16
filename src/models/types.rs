use std::collections::HashMap;
/// types.rs
/// @author bradlet

use crate::helpers;

#[derive(Debug)]
pub enum Program {
    Default,
    Main,
    MedMode
}

impl Program {
    // Rust doesn't seem to have something like ".values()" on enums by default or as part of
    // the standard library, so just making this to have access to a list of variants.
    pub fn programs() -> [Program; 3] {
        [Program::Default, Program::Main, Program::MedMode]
    }

    pub fn from(name: &str) -> Program {
        match name {
            "default" => Program::Default,
            "main" => Program::Main,
            "mm" => Program::MedMode,
            _ => Program::Default,
        }
    }

    pub fn run(name: String, args: Vec<String>) -> () {
        println!("Running program (default to main): {}", name);
        helpers::horizontal_sep();

        let used = match Program::from(&name) {
            // In default case
            Program::Default => Program::main( args),
            Program::Main => Program::main(args),
            Program::MedMode => Program::median_mode_from_args(args),
        };

        helpers::horizontal_sep();
        println!("Completed call to {} with args: {}", name, used.join(", ").as_str())
    }


    fn main(args: Vec<String>) -> Vec<String> {
        println!("Welcome to Main! Running...");
        fn take_val(str: String) -> String {
            println!("I stole this string: {}", str);
            str
        }
        let test_ownership = String::from("test");
        let returned_ownership = take_val(test_ownership);
        println!("string is not gone? {}", returned_ownership);

        let option: Option<&String> = args.get(2);

        match option {
            Some(str) => println!("You sneaky B, you included {}?!?!", str),
            None => println!("Everything's running fine, nothing else to see here."),
        };

        args
    }

    fn median_mode_from_args(args: Vec<String>) -> Vec<String> {
        // First index is rust exec name, second is selected program name;
        // the rest are assumed to be positive integers.
        let mut int_args: Vec<u32> = args[2..].iter()
            .map( |x|
                x.parse::<u32>().unwrap()
            )
            .collect();

        // Median:
        int_args.sort();
        println!("The median of this list is {}", int_args[int_args.len() / 2]);

        // Mode:
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for i in &int_args {
            counts.entry(*i).and_modify(|i| *i += 1).or_insert(1);
        }


        let mut max_count: u32 = 0;
        let mut max_index: u32 = 0;
        for (index, count) in counts.iter() {
            if *count > max_count {
                max_count = *count; // Both are primitive, so they have the Copy trait.
                max_index = *index; // SO, cur_max and max_index are managed on the stack.
            }
        }

        println!("The mode of this list occurs {} times and is {}.", max_count, max_index);

        args
    }

}
