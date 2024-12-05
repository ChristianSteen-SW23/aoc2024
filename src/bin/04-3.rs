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
    avg_time_for_fn(50000, "2", part2, "src/input/04/owndata2.txt");
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

fn cal_part2(data: (Vec<u8>, i32)) -> i32 {
    let (arr, width) = data;
    let mut xmas_count: i32 = 0;
    for i in width..(arr.len() as i32 - width) {
        if i % width != 0 && i % width != width - 1 {
            let count = check_one_spot_p2_org((&arr, width, i));
            xmas_count += count;
        }
    }
    xmas_count
}

fn check_one_spot_p2_org(data: (&[u8], i32, i32)) -> i32 {
    let (arr, width, spot) = data;

    if arr.get(spot as usize).unwrap() == &b'A' {
        if !((arr.get((spot - width - 1) as usize).unwrap() == &b'M'
            && arr.get((spot + width + 1) as usize).unwrap() == &b'S')
            || (arr.get((spot - width - 1) as usize).unwrap() == &b'S'
                && arr.get((spot + width + 1) as usize).unwrap() == &b'M'))
        {
            return 0;
        }
        if (arr.get((spot - width + 1) as usize).unwrap() == &b'M'
            && arr.get((spot + width - 1) as usize).unwrap() == &b'S')
            || (arr.get((spot - width + 1) as usize).unwrap() == &b'S'
                && arr.get((spot + width - 1) as usize).unwrap() == &b'M')
        {
            return 1;
        }
    }
    0
}

fn check_one_spot_p2(data: (&[u8], i32, i32)) -> i32 {
    let (arr, width, spot) = data;

    if arr.get(spot as usize).unwrap() != &b'A' {
        return 0;
    }

    let top_left_index = arr.get((spot - width - 1) as usize).unwrap();
    let bottom_right_index = arr.get((spot + width + 1) as usize).unwrap();
    

    if !((top_left_index == &b'M'
        && bottom_right_index == &b'S')
        || (top_left_index == &b'S'
            && bottom_right_index == &b'M'))
    {
        return 0;
    }
    let top_right_index = arr.get((spot - width + 1) as usize).unwrap();
    let bottom_left_index = arr.get((spot + width - 1) as usize).unwrap();
    if !((top_right_index == &b'M'
        && bottom_left_index == &b'S')
        || (top_right_index == &b'S'
            && bottom_left_index == &b'M'))
    {
        return 0;
    }
    1
}

fn read_file_data_vec(file_name: String) -> Result<(Vec<u8>, i32), io::Error> {
    let content = fs::read(&file_name)?; // Read the file as bytes

    // Filter out newline bytes (`\n` or `\r\n`) and create a Vec<u8>
    let arr1: Vec<u8> = content
        .into_iter()
        .filter(|&c| c != b'\n' && c != b'\r')
        .collect();

    // Get the width as the length of the first line, if available
    let width = 140;
    Ok((arr1, width))
}
