use colored::*;

pub fn show(msg: &str) {
    let error_msg = format!("\n\n{} \n\n", msg.red().bold());
    println!("{}", error_msg);
}