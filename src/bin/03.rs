use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    //let ting = part2("src/input/03/bigdata2.txt".to_string()).unwrap_or(0);
    //println!("{}", ting);

    print_solution()
}

fn part1(file_name: String) -> Result<i32, io::Error> {
    let file = File::open(&file_name)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader
    let re = Regex::new(r"^(\d+),(\d+)\)").unwrap();
    let mut total_count: i32 = 0;
    for line in reader.lines() {
        let mut line = line?;
        loop {
            match line.find("mul(") {
                Some(index) => {
                    // Split off the part after "mul("
                    line = line.split_off(index + 4);
                    if let Some(captures) = re.captures(&line) {
                        let num1 = captures.get(1).unwrap().as_str(); // First captured group
                        let num2 = captures.get(2).unwrap().as_str(); // Second captured group
                        total_count += num1.parse().unwrap_or(0) * num2.parse().unwrap_or(0);
                    }
                }
                None => break,
            }
        }
    }
    Ok(total_count)
}

fn part2(file_name: String) -> Result<i32, io::Error> {
    let file = File::open(&file_name)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader
    let re = Regex::new(r"^(\d+),(\d+)\)").unwrap();
    let mut total_count: i32 = 0;
    let mut is_it_do = true;

    for line in reader.lines() {
        let mut line = line?;
        loop {
            match line.find("mul(") {
                Some(index) => {
                    match line.find("do") {
                        Some(index_do) => {
                            if index_do < index {
                                //TODO update do

                                line = line.split_off(index_do + 1);
                                //println!("{}", line);
                                match &line {
                                    s if s.to_string().starts_with("o()") => {
                                        is_it_do = true;
                                    }
                                    s if s.to_string().starts_with("on't()") => {
                                        is_it_do = false;
                                    }
                                    _ => {}
                                }
                                continue;
                            }
                        }
                        None => {}
                    }
                    // Split off the part after "mul("
                    line = line.split_off(index + 4);
                    // Call the function to process the line and handle the captures
                    if let Some(product) = process_mul_case(&line, &re)? {
                        if is_it_do {
                            //println!("{}", product);
                            total_count += product;
                        }
                    }
                }
                None => break,
            }
        }
    }

    Ok(total_count)
}

// Function to process the part after "mul(" in the line and return the product of the numbers
fn process_mul_case(line: &str, re: &Regex) -> Result<Option<i32>, io::Error> {
    if let Some(captures) = re.captures(line) {
        let num1 = captures.get(1).unwrap().as_str(); // First captured group
        let num2 = captures.get(2).unwrap().as_str(); // Second captured group

        let num1_i32 = num1.parse::<i32>().unwrap_or(0); // Convert to i32, default to 0 if parsing fails
        let num2_i32 = num2.parse::<i32>().unwrap_or(0); // Convert to i32, default to 0 if parsing fails

        //println!("Number before comma: {}", num1);
        //println!("Number after comma: {}", num2);

        // Return the product of the two numbers as Some(i32)
        Ok(Some(num1_i32 * num2_i32))
    } else {
        Ok(None)
    }
}

fn print_solution() {
    let timer = Instant::now();
    time_solution(
        "1",
        part1("src/input/03/bigdata.txt".to_string()).unwrap_or(0),
        timer.elapsed().as_micros(),
    );

    let timer = Instant::now();
    time_solution(
        "2",
        part2("src/input/03/bigdata2.txt".to_string()).unwrap_or(0),
        timer.elapsed().as_micros(),
    );
}

fn time_solution(part: &str, solution: i32, time: u128) {
    println!(
        "The solution to part {} is: {}\nFinished in: {}µs",
        part, solution, time
    )
}

#[cfg(test)]
mod day3 {
    use super::*;
    const INPUT: &str = "src/input/03/test.txt";
    const INPUT2: &str = "src/input/03/bigdata.txt";

    const INPUT3: &str = "src/input/03/test2.txt";
    const INPUT4: &str = "src/input/03/bigdata2.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()).unwrap(), 161);
    }
    #[test]
    fn test_part1_big() {
        assert_eq!(part1(INPUT2.to_string()).unwrap(), 184576302);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT3.to_string()).unwrap(), 48);
    }
    #[test]
    fn test_part2_big() {
        assert_eq!(part2(INPUT4.to_string()).unwrap(), 118173507);
    }
}
