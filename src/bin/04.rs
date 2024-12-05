use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::usize;

fn main() {
    //println!("RESULT {}", part1("src/input/04/owndata.txt".to_string()));
    //part1("src/input/04/owndata.txt".to_string());
    //print_solution();
    //avg_time_for_fn(10000, "1", part1, "src/input/04/owndata.txt");
    //avg_time_for_fn(1, "2", part2, "src/input/04/test.txt");
    avg_time_for_fn(50000, "2", part2, "src/input/04/owndata.txt");

}

fn part1(file_name: String) -> i32 {
    match read_file_data_holefile(file_name.to_owned()) {
        Ok((arr, width)) => {
            return cal_part1((arr, width));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    0
}

fn part2(file_name: String) -> i32 {
    match read_file_data_vec(file_name.to_owned()) {
        Ok((arr, width)) => {
            //println!("The read data is: {:?},{}", arr, width);
            return cal_part2((arr, width));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    0
}

fn cal_part1(data: (Vec<char>, i32)) -> i32 {
    let (arr, width) = data;
    let mut xmas_count: i32 = 0;
    for i in 0..arr.len() {
        let count = check_one_spot_p1((&arr, width, i));
        xmas_count += count;
    }

    xmas_count
}

fn check_one_spot_p1(data: (&[char], i32, usize)) -> i32 {
    let (arr, width, spot) = data;
    let spot: i32 = spot as i32;
    let mut count = 0;
    let add_arr = [
        1,
        -1,
        -width - 1,
        width - 1,
        width + 1,
        -width + 1,
        width,
        -width,
    ];
    let add_row_arr = [0, 0, -1, 1, 1, -1, 1, -1];
    for j in 0..add_arr.len() {
        let add = add_arr[j];
        for i in 0..5 {
            let cur_char: char;
            match i {
                0 => cur_char = 'X',
                1 => cur_char = 'M',
                2 => cur_char = 'A',
                3 => cur_char = 'S',
                _ => {
                    count += 1;
                    break;
                }
            }
            // Check if skip row
            if (spot - spot % width) / width + i * add_row_arr[j]
                != ((spot + i * (add)) - ((spot + i * (add)) % width)) / width
            {
                break;
            }
            // Check for same char
            if arr.get((spot + i * add) as usize).unwrap_or(&'n') != &cur_char {
                break;
            }
        }
    }
    //println!("{}", count);
    count
}

fn cal_part2(data: (Vec<char>, i32)) -> i32 {
    let (arr, width) = data;
    let mut xmas_count: i32 = 0;
    for i in width..arr.len() as i32 - width {
        if i % width != 0 && i % width != width - 1 {
            //println!("{}", i);
            let count = check_one_spot_p2((&arr, width, i));
            xmas_count += count;
        }
    }

    xmas_count
}

fn check_one_spot_p2(data: (&[char], i32, i32)) -> i32 {
    let (arr, width, spot) = data;

    if arr.get(spot as usize).unwrap() == &'A' {
        if !((arr.get((spot - width - 1) as usize).unwrap() == &'M'
            && arr.get((spot + width + 1) as usize).unwrap() == &'S')
            || (arr.get((spot - width - 1) as usize).unwrap() == &'S'
                && arr.get((spot + width + 1) as usize).unwrap() == &'M'))
        {
            return 0;
        }
        if (arr.get((spot - width + 1) as usize).unwrap() == &'M'
            && arr.get((spot + width - 1) as usize).unwrap() == &'S')
            || (arr.get((spot - width + 1) as usize).unwrap() == &'S'
                && arr.get((spot + width - 1) as usize).unwrap() == &'M')
        {
            return 1;
        }
    }
    0
}


fn read_file_data(file_name: String) -> Result<(Vec<char>, i32), io::Error> {
    // Read a file line for line
    let file = File::open(&file_name)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    let mut arr1: Vec<char> = Vec::new();
    let mut line2: String = "2".to_string();
    for line in reader.lines() {
        line2 = line?; // Unwrap the Result to get the line as a string
        arr1.extend(line2.chars());
    }

    let width = line2.len() as i32;

    Ok((arr1, width))
}

fn read_file_data_holefile(file_name: String) -> Result<(Vec<char>, i32), io::Error> {
    // Read the entire content of the file into a string
    let content = fs::read_to_string(&file_name)?;
    let content = content.replace("\n", "");
    // Convert the string into a Vec<char>
    let arr: Vec<char> = content.chars().collect();

    let width = (arr.len() as f64).sqrt() as i32;

    Ok((arr, width))
}

fn read_file_data_vec(file_name: String) -> Result<(Vec<char>, i32), io::Error> {
    let content = fs::read_to_string(&file_name)?;

    // Pre-allocate space for `Vec<char>` to avoid multiple reallocations
    let mut arr1: Vec<char> = content.chars().filter(|&c| c != '\n').collect();
    
    // Get the width as the length of the first line, if available
    let width = content.lines().next().map(|line| line.len() as i32).unwrap_or(0);

    Ok((arr1, width))
}




fn avg_time_for_fn(run_amount: i32, part: &str, part_fn: fn(String) -> i32, file: &str) {
    let mut time_arr: Vec<u128> = Vec::new(); // Use a Vec for dynamic size
    let mut result: i32 = 0;

    for _ in 0..run_amount {
        let timer = Instant::now();
        result = part_fn(file.to_string());
        time_arr.push(timer.elapsed().as_micros());
    }

    // Calculate the average time
    let total_time: u128 = time_arr.iter().sum();
    let average_time: f64 = total_time as f64 / run_amount as f64;

    println!(
        "The solution to part {} is: {}\nFinished in avg: {}µs over {} runs, {}s total time",
        part,
        result,
        average_time.round(),
        run_amount,
        total_time / 1_000_000
    );
}

// fn print_solution() {
//     let timer = Instant::now();
//     time_solution(
//         "2",
//         part2("src/input/04/owndata2.txt".to_string()),
//         timer.elapsed().as_micros(),
//     );
//     let timer = Instant::now();
//     time_solution(
//         "1",
//         part1("src/input/04/owndata.txt".to_string()),
//         timer.elapsed().as_micros(),
//     );
// }

// fn time_solution(part: &str, solution: i32, time: u128) {
//     println!(
//         "The solution to part {} is: {}\nFinished in: {}µs",
//         part, solution, time
//     )
// }
