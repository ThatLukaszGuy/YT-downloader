use std::process;
use crate::helpers::errortext::show;

pub fn dir_check(path: &str) {
    match project_root::get_project_root() {
        Ok(p) => {
            let s = p.to_str().unwrap_or_else(||{
                show("Could not convert root to string... Sorry");
                process::exit(1);
            });
            let x = format!("{}{}", s, path);
            std::fs::create_dir_all(&x).unwrap_or_else(|_x|{
                show("Could not create needed directories for output.. Maybe try running as administrator?");
                process::exit(1);
            });
        },
        _ => ()
    };
}
