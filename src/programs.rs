/// programs.rs
/// @author bradlet
use crate::helpers;
use std::collections::HashMap;

mod program_main;
mod program_med_mode;

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
            Program::Default => program_main::main( args),
            Program::Main => program_main::main(args),
            Program::MedMode => program_med_mode::main(args),
        };

        helpers::horizontal_sep();
        println!("Completed call to {} with args: {}", name, used.join(", ").as_str())
    }
}
