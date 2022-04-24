/// programs_main.rs
/// @author bradlet

pub fn main(args: Vec<String>) -> Vec<String> {
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
