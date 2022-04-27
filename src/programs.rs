/// programs.rs
/// @author bradlet
use crate::helpers;
use std::collections::HashMap;

mod program_main;
mod program_med_mode;
mod program_displayable;

#[derive(Debug)]
pub enum Program {
    Default,
    Main,
    MedMode,
    Displayable
}

impl Program {
    // Rust doesn't seem to have something like ".values()" on enums by default or as part of
    // the standard library, so just making this to have access to a list of variants.
    pub fn programs() -> [Program; 4] {
        [Program::Default, Program::Main, Program::MedMode, Program::Displayable]
    }

    pub fn from(name: &str) -> Program {
        match name {
            "default" => Program::Default,
            "main" => Program::Main,
            "mm" => Program::MedMode,
            "display" => Program::Displayable,
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
            Program::Displayable => program_displayable::main(args)
        };

        helpers::horizontal_sep();
        println!("Completed call to {} with args: {}", name, used.join(", ").as_str())
    }
}
