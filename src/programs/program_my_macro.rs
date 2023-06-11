//! program_my_macro.rs
//! @author bradlet

use postgres::{Client, NoTls};
use postgres_flatten::{flattened::ToFlattenedSql, ToFlattenedSql};
use text_colorizer::{Color, Colorize};

#[allow(dead_code)]
struct Health {
    weight: f32,
    age: u8,
}

#[allow(dead_code)]
#[derive(ToFlattenedSql)]
struct Cat {
    name: String,
    color: Color,
    health_data: Health,
}

pub fn main(args: Vec<String>) -> Vec<String> {
    println!("{}", "Welcome to My Macro Demo!".bright_cyan());

    let mut client = Client::connect(
        "host=localhost dbname=postgres port=32768 user=brad password=password",
        NoTls,
    )
    .unwrap();

    for row in client.query("SELECT * FROM test;", &[]).unwrap() {
        let name: &str = row.get(0);
        let age: i32 = row.get(1);
        let msg = format!("{} is {} years old!", name, age).bright_magenta();
        println!("{}", msg);
    }

    Cat::into_flattened_row();

    args
}
