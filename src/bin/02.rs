use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    print_solution();
}

fn part1(file_name: String) -> Result<i32, io::Error> {
    let file = File::open(&file_name)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    let mut count_of_safe: i32 = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line as a string
        let parts: Vec<&str> = line.split_whitespace().collect();

        let numbers: Vec<i32> = parts.iter().map(|&s| s.parse::<i32>().unwrap()).collect();

        let mut flag_safe = true;
        let flag_decrease: bool = numbers[0] - numbers[1] < 0;

        for i in 1..parts.len() {
            if numbers[i - 1] == numbers[i] {
                flag_safe = false;
                break;
            }

            if (numbers[i - 1] - numbers[i]).abs() > 3 {
                flag_safe = false;
                break;
            }

            if (numbers[i - 1] - numbers[i] > 0 && flag_decrease)
                || (numbers[i - 1] - numbers[i] < 0 && !flag_decrease)
            {
                flag_safe = false;
                break;
            }
        }
        if flag_safe {
            count_of_safe += 1;
        }
    }

    Ok(count_of_safe)
}

fn part2(file_name: String) -> Result<i32, io::Error> {
    let file = File::open(&file_name)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    let mut count_of_safe: i32 = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line as a string
        let parts: Vec<&str> = line.split_whitespace().collect();

        let num: Vec<i32> = parts.iter().map(|&s| s.parse::<i32>().unwrap()).collect();

        let mut flag_safe = true;
        let flag_decrease: bool = num[0] - num[num.len() - 1] < 0;
        let mut flag_skipped_step: bool = false;

        let mut i = 1;
        while i < parts.len() {
            if !part2_validater(num[i - 1], num[i], flag_decrease) {
                i += 1;
                continue;
            }
            if !flag_skipped_step {
                flag_skipped_step = true;
                if i + 1 == num.len() || !part2_validater(num[i - 1], num[i + 1], flag_decrease) {
                    i += 2;
                    continue;
                }
                if i - 1 == 0 || !part2_validater(num[i - 2], num[i], flag_decrease) {
                    i += 1;
                    continue;
                }
            }

            flag_safe = false;
            break;
        }
        if flag_safe {
            count_of_safe += 1;
        }
    }

    Ok(count_of_safe)
}

fn part2_validater(var1: i32, var2: i32, flag_decrease: bool) -> bool {
    if var1 == var2 {
        return true;
    }

    if (var1 - var2).abs() > 3 {
        return true;
    }

    if (var1 - var2 > 0 && flag_decrease) || (var1 - var2 < 0 && !flag_decrease) {
        return true;
    }
    false
}

fn print_solution() {
    let timer = Instant::now();
    match part1("src/input/02/owndata.txt".to_owned()) {
        Ok(file_data) => {
            time_solution("1", file_data, timer.elapsed().as_micros());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let timer = Instant::now();
    match part2("src/input/02/owndata.txt".to_owned()) {
        Ok(file_data) => {
            time_solution("2", file_data, timer.elapsed().as_micros());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn time_solution(part: &str, solution: i32, time: u128) {
    println!(
        "The solution to part {} is: {}\nFinished in: {}µs",
        part, solution, time
    )
}

#[cfg(test)]
mod day2 {
    use super::*;
    const INPUT: &str = "src/input/02/test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()).unwrap(), 4);
    }
}
