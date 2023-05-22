/// program_main.rs
/// @author bradlet
use text_colorizer::Colorize;
use postgres::{Client, NoTls};

pub fn main(args: Vec<String>) -> Vec<String> {
    let welcome = "Welcome to Main! Let's see what's in the DB...".bright_cyan();
    println!("{}", welcome);

    let mut client = Client::connect("host=localhost dbname=postgres port=32768 user=brad password=password", NoTls).unwrap();
    for row in client.query("SELECT * FROM test;", &[]).unwrap() {
        let name: &str = row.get(0);
        let age: i32 = row.get(1);
        let msg = format!("{} is {} years old!", name, age).bright_magenta();
        println!("{}", msg);
    }

    args
}
