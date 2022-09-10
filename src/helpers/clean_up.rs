use std::process;
use crate::helpers::errortext::show;

pub fn clean_up_files(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path).unwrap_or_else(|_x|{
        show("Could not remove/clean up temporary file.. Maybe try running as administrator?");
        process::exit(1);
    });
    Ok(())
}