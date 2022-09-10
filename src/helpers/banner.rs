use colored::*;

pub fn banner() {
    const BANNER: &str = r"
    __   __ _____      ___                      _                _ 
    \ \ / /|_   _|___ |   \  ___ __ __ __ _ _  | | ___  __ _  __| |
     \ V /   | | |___|| |) |/ _ \\ V  V /| ' \ | |/ _ \/ _` |/ _` |
      |_|    |_|      |___/ \___/ \_/\_/ |_||_||_|\___/\__,_|\__,_|
                
               By url download any YouTube Video you want.
                                                         ";

    println!("{}", BANNER.blue().bold());
}