use std::io;

fn is_safe(nums: &Vec<i32>) -> bool {
    let increasing: bool = nums[0] < nums[1];

    for i in 0..nums.len()-1 {
        let dist: i32 = nums[i+1] - nums[i];

        if dist.abs() < 1 || dist.abs() > 3 {
            return false
        } else if dist > 0 && !increasing {
            return false
        } else if dist < 0 && increasing {
            return false
        }
    }

    return true
}

fn main() {
    let mut count: u32 = 0;

    let lines = io::stdin().lines();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        let nums: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        if nums.len() == 0 {
            break;
        }

        if is_safe(&nums) {
            count += 1;
        }
    }

    println!("{}", count);
}