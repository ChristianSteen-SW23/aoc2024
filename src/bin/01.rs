use std::fs;
use std::io::{self, BufRead};
use std::fs::File;

struct BothList {
    arr1: Vec<i32>,
    arr2: Vec<i32>,
}

fn main() {
    println!("Hello, world!");
    let mut fileData;
    match readFileData("./../input/01/owndata.txt".to_owned()) {
        Ok(both_list) => {
            fileData = both_list;
        println!("\n\n\nThe result is: {}",calculateResultPart2(&mut fileData));
        }
        Err(e) => eprintln!("Error: {}", e),
    }


}

fn calculateResultPart1(both_list: &mut BothList) -> i32 {
    let mut result: i32 = 0;
    both_list.arr1.sort();
    both_list.arr2.sort();
    for i in 0..both_list.arr1.len(){
        print!("{}, {}\n",both_list.arr1[i],both_list.arr2[i]);
        result += (both_list.arr1[i] - both_list.arr2[i]).abs();
    }
    result
}

fn calculateResultPart2(both_list: &mut BothList) -> i32 {
    let mut result: i32 = 0;
    both_list.arr1.sort();
    both_list.arr2.sort();
    for i in 0..both_list.arr1.len(){
        result += (both_list.arr1[i] * both_list.arr2.iter().filter(|&x| *x == both_list.arr1[i]).count() as i32);
    }
    result
}

fn readFileData(fileName: String) -> Result<BothList, io::Error> {
    // Read the file as a string
    let content = fs::read_to_string(&fileName)?;
    println!("File content:\n{}", content);

    println!("\n\n");
    // Read a file line for line
    let file = File::open(&fileName)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line as a string
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            arr1.push(parts[0].parse::<i32>().unwrap());
            arr2.push(parts[1].parse::<i32>().unwrap());
        }
    }

    Ok(BothList { arr1, arr2 })
}