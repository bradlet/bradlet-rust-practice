//! program_my_macro.rs
//! @author bradlet

use postgres_flatten::{flattened::ToFlattenedSql, ToFlattenedSql};
use text_colorizer::{Color, Colorize};

#[derive(ToFlattenedSql)]
struct Cat {
    name: String,
    age: u8,
    color: Color,
}

pub fn main(args: Vec<String>) -> Vec<String> {
    println!("{}", "Welcome to My Macro Demo!".bright_cyan());

    let test = Cat {
        name: String::from("Rudy"),
        age: 2,
        color: Color::White
    };

    Cat::into_flattened_row();

    args
}
