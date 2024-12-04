use std::fs;
use regex::Regex;

fn main() {
    let mut total = 0;
    let line = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();
    let mut enabled : bool = true;

    for capture in re.captures_iter(&line) {
        match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if capture[0].starts_with("mul(") && enabled => {
                total += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
            },
            _ => {},
        }
    }
    
    println!("{}", total);
}