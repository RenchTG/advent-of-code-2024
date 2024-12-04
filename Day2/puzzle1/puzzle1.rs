use std::io;

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

        if good {
            count += 1;
        }
    }

    println!("{}", count);
}