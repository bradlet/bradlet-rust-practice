use bradleys_random_rust_helpers::horizontal_sep;
use text_colorizer::Color;

mod program_displayable;
/// programs.rs
/// @author bradlet
mod program_main;
mod program_med_mode;

#[derive(Debug, PartialEq)]
pub enum Program {
    Default,
    Main,
    MedMode,
    Displayable,
}

impl Program {
    // Rust doesn't seem to have something like ".values()" on enums by default or as part of
    // the standard library, so just making this to have access to a list of variants.
    pub fn programs() -> [Program; 4] {
        [
            Program::Default,
            Program::Main,
            Program::MedMode,
            Program::Displayable,
        ]
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
        horizontal_sep(40, Some(Color::BrightGreen));

        let used = match Program::from(&name) {
            // In default case
            Program::Default => program_main::main(args),
            Program::Main => program_main::main(args),
            Program::MedMode => program_med_mode::main(args),
            Program::Displayable => program_displayable::main(args),
        };

        horizontal_sep(40, Some(Color::BrightGreen));
        println!(
            "Completed call to {} with args: {}",
            name,
            used.join(", ").as_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Program;
    /*
    Could also do:
    > use super::*;
    That would import all modules in the same file basically.
     */

    #[test]
    fn returns_expected_program() {
        assert_eq!(Program::Default, Program::from("default"));
        assert_eq!(Program::Main, Program::from("main"));
        assert_eq!(Program::MedMode, Program::from("mm"));
        assert_eq!(Program::Displayable, Program::from("display"));
        // Unexpected values get default
        assert_eq!(Program::Default, Program::from("SomethingElse"));
    }

    #[test]
    #[should_panic]
    #[ignore] // Now this will only run if it is specifically targeted with `cargo test __target__`
    fn test_out_should_panic() {
        panic!("Oh no!");
    }
}
