/// types.rs
/// @author bradlet

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
        println!("Running program (or default): {}\n", name);
        let used = match Program::from(&name) {
            // In default case
            Program::DEFAULT => Program::main( args),
            Program::MAIN => Program::main(args),
        };
        println!("Completed call to {} with args: {}", name, used.join(", ").as_str())
    }


    pub fn main(args: Vec<String>) -> Vec<String> {
        for arg in args.iter() {
            println!("arg: {}\n", arg)
        }
        args
    }
}
