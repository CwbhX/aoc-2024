use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let filename = "src/input.txt";


    // File opening handling
    let file = match File::open(filename){
        Ok(file) => file, // If file is opened successfully then return the unwrapped file
        Err(error) => {   // Otherwise handle the error
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };

    // Buffered reader object
    let reader = BufReader::new(file);

    // Vector of number pairs
    let mut number_pairs: Vec<[i32; 2]> = Vec::new();
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();

    // Iterate through the read lines
    for (line_number, line_data_result) in reader.lines().enumerate(){
        // Attempt to read lines
        let line = match line_data_result {
            Ok(line_data) => line_data,
            Err(error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, error);
                continue;
            }
        };

        // Split the line up into the 2 number strings
        let number_strings: Vec<&str> = line.split_whitespace().collect();

        // Make sure there are at least 2 numbers per row before working on them, if there aren't skip this row
        if number_strings.len() != 2 {
            eprintln!("Invalid line {}: Expected 2 numbers, found {}", 
                      line_number + 1, number_strings.len());
            continue;
        }

        // Parse n cast first number
        let first = match number_strings[0].parse::<i32>() {
            Ok(num) => num,
            Err(error) => {
                eprintln!("Error parsing first number on line {}: {}", 
                          line_number + 1, error);
                continue;
            }
        };

        // Parse n cast the second number
        let second = match number_strings[1].parse::<i32>() {
            Ok(num) => num,
            Err(error) => {
                eprintln!("Error parsing second number on line {}: {}", 
                          line_number + 1, error);
                continue;
            }
        };

        // Add these to our pair vector
        number_pairs.push([first, second]);
        array1.push(first);
        array2.push(second);
    }

    array1.sort();
    array2.sort();

    if array1.len() != array2.len() {
        panic!("Vectors are not the same length");
    }

    let mut sum_array = Vec::new();
    let mut total_distance: i32 = 0;

    for index in 0..array1.len() {
        let distance = (array1[index] - array2[index]).abs();
        sum_array.push(distance);
        total_distance += distance;
    }

    // Print out the successfully imported number pairs
    // println!("Successfully imported {} number pairs:", number_pairs.len());
    // for (index, pair) in number_pairs.iter().enumerate() {
    //     println!("Line {}: [{}, {}]", index + 1, pair[0], pair[1]);
    // }

    // // Optional: Additional information about parsing
    // if number_pairs.is_empty() {
    //     println!("No valid number pairs were found in the file.");
    // }

    println!("Total Distance is {}", total_distance);

}
