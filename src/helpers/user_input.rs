use colored::Colorize;



pub fn get_input(prompt: &str) -> String{
    use std::io;
    println!("{}",prompt.cyan());
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => (),
        Err(_no_updates_is_fine) => (),
    }
    input.trim().to_string()
}