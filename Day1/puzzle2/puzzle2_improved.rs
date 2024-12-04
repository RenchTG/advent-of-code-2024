use std::io;
use std::collections::HashMap;

fn main() {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let mut occurrence: HashMap<u32, u32> = HashMap::new();

    let lines = io::stdin().lines();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        let nums: Vec<&str> = line.trim().split_whitespace().collect();

        if nums.len() != 2 {
            break;
        }

        let l: u32 = nums[0].parse().unwrap();
        let r: u32 = nums[1].parse().unwrap();

        left.push(l);
        right.push(r);
        *occurrence.entry(r).or_insert(0) += 1;
    }

    let mut sum: u32 = 0;
    for i in 0..left.len() {
        sum += left[i] * occurrence.get(&left[i]).unwrap_or(&0);
    }
    println!("{}", sum);
}