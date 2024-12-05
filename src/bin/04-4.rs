use std::fs::{self};
use std::io::{self};
use std::time::Instant;
use std::usize;
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
fn main() {
    //avg_time_for_fn(1, "2", part2, "src/input/04/test.txt");
    avg_time_for_fn(50000, "2", part2, "src/input/04/owndata.txt");

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



fn read_file_data_vec(file_name: String) -> Result<(Vec<char>, i32), io::Error> {
    let content = fs::read_to_string(&file_name)?;

    // Pre-allocate space for Vec<char> to avoid multiple reallocations
    let arr1: Vec<char> = content.chars().filter(|&c| c != '\n').collect();
    
    // Get the width as the length of the first line, if available
    let width = content.lines().next().map(|line| line.len() as i32).unwrap_or(0);

    Ok((arr1, width))
}





