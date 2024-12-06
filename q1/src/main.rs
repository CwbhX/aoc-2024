use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn openInputs(input_filename: &str) -> (Vec<i32>, Vec<i32>){
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();

    let file = match File::open(input_filename){
        Ok(file) => file,
        Err(error) => {   // Otherwise handle the error
            eprintln!("Error opening file {}: {}", input_filename, error);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    for (line_number, line_data_result) in reader.lines().enumerate(){
        let line = match line_data_result {
            Ok(line_data) => line_data,
            Err(error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, error);
                continue;
            }
        };

        let line_number_strings: Vec<&str> = line.split_whitespace().collect();

        if line_number_strings.len() != 2 {
            eprintln!("Invalid line {}: Expected 2 numbers, found {}", 
            line_number + 1, line_number_strings.len());
            continue;
        }

        let first_num = match line_number_strings[0].parse::<i32>(){
            Ok(number) => number,
            Err(error) => {
                eprintln!("Error parsing first number on line {}: {}", 
                          line_number + 1, error);
                continue;
            }
        };

        let second_num = match line_number_strings[1].parse::<i32>(){
            Ok(number) => number,
            Err(error) => {
                eprintln!("Error parsing first number on line {}: {}", 
                          line_number + 1, error);
                continue;
            }
        };

        array1.push(first_num);
        array2.push(second_num);
    }

    return (array1, array2);
}

fn partone(){
    let filename = "src/input.txt";

    let (mut array1, mut array2) = openInputs(filename);


    // // File opening handling
    // let file = match File::open(filename){
    //     Ok(file) => file, // If file is opened successfully then return the unwrapped file
    //     Err(error) => {   // Otherwise handle the error
    //         eprintln!("Error opening file {}: {}", filename, error);
    //         std::process::exit(1);
    //     }
    // };

    // // Buffered reader object
    // let reader = BufReader::new(file);

    // // Vector of number pairs
    // let mut number_pairs: Vec<[i32; 2]> = Vec::new();
    // let mut array1: Vec<i32> = Vec::new();
    // let mut array2: Vec<i32> = Vec::new();

    // // Iterate through the read lines
    // for (line_number, line_data_result) in reader.lines().enumerate(){
    //     // Attempt to read lines
    //     let line = match line_data_result {
    //         Ok(line_data) => line_data,
    //         Err(error) => {
    //             eprintln!("Error reading line {}: {}", line_number + 1, error);
    //             continue;
    //         }
    //     };

    //     // Split the line up into the 2 number strings
    //     let number_strings: Vec<&str> = line.split_whitespace().collect();

    //     // Make sure there are at least 2 numbers per row before working on them, if there aren't skip this row
    //     if number_strings.len() != 2 {
    //         eprintln!("Invalid line {}: Expected 2 numbers, found {}", 
    //                   line_number + 1, number_strings.len());
    //         continue;
    //     }

    //     // Parse n cast first number
    //     let first = match number_strings[0].parse::<i32>() {
    //         Ok(num) => num,
    //         Err(error) => {
    //             eprintln!("Error parsing first number on line {}: {}", 
    //                       line_number + 1, error);
    //             continue;
    //         }
    //     };

    //     // Parse n cast the second number
    //     let second = match number_strings[1].parse::<i32>() {
    //         Ok(num) => num,
    //         Err(error) => {
    //             eprintln!("Error parsing second number on line {}: {}", 
    //                       line_number + 1, error);
    //             continue;
    //         }
    //     };

    //     // Add these to our pair vector
    //     number_pairs.push([first, second]);
    //     array1.push(first);
    //     array2.push(second);
    // }

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

fn parttwo(){
    let (mut array1, mut array2) = openInputs("src/input.txt");

    // Create empty hashmap for the counts for each number
    let mut number_counts: HashMap<i32, i32> = HashMap::new();

    // For loop for the main array but dereference the number from the array and borrow the array so it doesn't go out of scope
    for &number in &array1 {
        // Get an iterator of all the matching numbers of the current number in the second array
        // Iter gives us references to the items, filter takes a closure where x is the parameter that needs to be deref twice from iter and filter, then we have the filter closure
        // Iterators don't do anything by default, need to call a method on them
        let matching_numbers = array2.iter().filter(|&&x| x == number);

        // Get the count of that filter result for this current number
        let count = matching_numbers.count();
        println!("Current number: {}, seen in array 2: {} times", number, count); // Don't need to worry about borrowing count since it is a primitive type and implements copy

        // If the count isn't zero, add this to the hashmap
        if count != 0{
            number_counts.insert(number, count as i32);
        }
    }

    let mut total_sum:i32 = 0;

    // Iterate over the hashmap to perform the mathymaths
    for (number, count) in &number_counts{
        // since number and count are given has references we need to derefernce them first with * before using them (println can handle references)
        total_sum += *number * *count;
    }

    println!("Total sum: {}", total_sum);

}

fn main(){
    partone();
    parttwo();
}
