use std::io;
use regex::Regex;

fn main() {
    let mut total: i32 = 0;

    let lines = io::stdin().lines();
    for line in lines {
        let line = match line {
            Ok(line) => if line.is_empty() { break } else { line },
            Err(_) => break,
        };

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        for (_, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
            total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }
    
    println!("{}", total);
}