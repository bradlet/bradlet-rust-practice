/// types.rs
/// @author bradlet

use crate::helpers;

#[derive(Debug)]
pub enum Program {
    DEFAULT,
    MAIN,
}

impl Program {
    pub fn programs() -> Vec<Program> {
        vec![Program::DEFAULT, Program::MAIN]
    }

    pub fn from(name: &str) -> Program {
        match name {
            "default" => Program::DEFAULT,
            "main" => Program::MAIN,
            _ => Program::DEFAULT,
        }
    }

    pub fn run(name: String, args: Vec<String>) -> () {
        println!("Running program (default to main): {}", name);
        helpers::horizontal_sep();

        let used = match Program::from(&name) {
            // In default case
            Program::DEFAULT => Program::main( args),
            Program::MAIN => Program::main(args),
        };

        helpers::horizontal_sep();
        println!("Completed call to {} with args: {}", name, used.join(", ").as_str())
    }


    pub fn main(args: Vec<String>) -> Vec<String> {
        println!("Welcome to Main! Running...");

        for arg in args.iter() {
            println!("arg: {}", arg)
        }

        args
    }
}
