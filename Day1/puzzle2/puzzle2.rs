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
    let mut right_idx: usize = 0;

    for mut i in 0..left.len() {
        let mut count: i32 = 0;
        while right[right_idx] <= left[i] {
            if right[right_idx] == left[i] {
                count += 1;
            }
            right_idx += 1;
        }
        sum += left[i] * count;

        while i+1 < left.len() && left[i+1] == left[i] {
            i += 1;
            sum += left[i] * count;
        }
    }

    println!("{}", sum);
}