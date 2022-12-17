use std::time::{Duration, Instant};
use std::io;
use anyhow::Result;
use colored::*;


fn main() -> Result<()>{
    
    let start = Instant::now();
    let duration = Duration::new(60, 0);
    let  list_of_words = include_str!("words.txt")
        .lines()
        .map(|line| return line.trim())
        .collect::<Vec<&str>>();
    
    let mut words_per_min = 0;
    let mut input = String::new();
    while start.elapsed() < duration {
        for word in list_of_words.iter() {
        println!("Current word - {}",word.blue().bold());
        println!("{} seconds remaining", 
            duration.checked_sub(start.elapsed()).unwrap().as_secs()
        );
        println!("Words per minute - {}",words_per_min);
        std::thread::sleep(Duration::from_secs(1));
            io::stdin().read_line(& mut input).unwrap();
            if word == (&input.trim()) {
                println!("{}","Good!".green().bold());
                words_per_min += 1;
            } else {
                println!("{}","Wrong!".red().bold());
            }
            input.clear();
        }
    }

    Ok(())
}
