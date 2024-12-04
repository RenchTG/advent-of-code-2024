use std::io;

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut good: bool = true;
    // increasing
    if nums[0] < nums[1] {
        for i in 0..nums.len()-1 {
            if !(nums[i+1] - nums[i] >= 1 && nums[i+1] - nums[i] <= 3) {
                good = false;
                break;
            }
        }
    }
    // decreasing
    else if nums[0] > nums[1] {
        for i in 0..nums.len()-1 {
            if !(nums[i] - nums[i+1] >= 1 && nums[i] - nums[i+1] <= 3) {
                good = false;
                break;
            }
        }
    }
    // equal
    else {
        good = false;
    }

    return good
}

fn main() {
    let mut count: u32 = 0;

    let lines = io::stdin().lines();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        let mut nums: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        if nums.len() == 0 {
            break;
        }

        for i in 0..nums.len() {
            let copy: i32 = nums[i];
            nums.remove(i);

            if is_safe(&nums) {
                count += 1;
                break;
            }

            nums.insert(i, copy);
        }
    }

    println!("{}", count);
}