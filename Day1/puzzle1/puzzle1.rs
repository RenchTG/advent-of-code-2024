use std::io;

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

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

        left.push(nums[0].parse().unwrap());
        right.push(nums[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut sum: i32 = 0;

    for i in 0..left.len() {
        sum += i32::abs(left[i] - right[i]);
    }

    println!("{}", sum);
}