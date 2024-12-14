use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_inputs(input_filename: &str) -> Vec<Vec<i32>> {
    // Prep the internal datastructure for the data
    let mut levels_of_reports = Vec::<Vec<i32>>::new(); // Use turbofish, calling new on the Vec, but meanwhile specifying that that call will be using Vec<i32> for the elements

    // Safely open the input file
    let file = match File::open(input_filename){
        Ok(file) => file,
        Err(error) => {   // Otherwise handle the error
            eprintln!("Error opening file {}: {}", input_filename, error);
            std::process::exit(1);
        }
    };

    // Prepare a reader to parse the data in a buffer
    let reader = BufReader::new(file);

    // Parse safely through each line of the file by getting the buffer lines and then an enumeration object which we can for loop on
    for (line_number, line_data_result) in reader.lines().enumerate(){
        let line = match line_data_result{
            Ok(line_data) => line_data,
            Err(error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, error);
                continue;
            }
        };

        // Split up the numbers by whitespace, still in string section form (static strings)
        let line_number_strings: Vec<&str> = line.split_whitespace().collect();

        let report: Vec<i32> = line_number_strings.iter()
            .enumerate()
            .filter_map(|(index, str_num)| {
                match str_num.parse::<i32>() {
                    Ok(num) => Some(num),     // Wrapping in Some will cause the filter part to keep this in its final returned iterator
                    Err(error) => {           // Couldn't parse -> prints error
                        eprintln!("Error at {}: {}", index, error);
                        None                      // Returns None (gets filtered out)
                    }   
                }
            })
            .collect();

        // Add this "report" to the levels in our array of arrays
        levels_of_reports.push(report);
    }

    return levels_of_reports;
}

// Important to accept a reference so that the function borrows the input vector - I THINK. Otherwise I would need to perform an expensive copy/clone?
fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut is_safe = true; // Default to true unless proven otherwise
    let mut initial_ascending = true; // Default ascending to true prior to checking the first pair to not use optionals

    // If let's work on tuples which is fucking insane magic shit but lets do it
    if let (Some(&first_num), Some(&second_num)) = (report.first(), report.get(1)){ // Using the & in the Some, means that the variable we're creating in Some will match and copy the reference returned to us
        if first_num - second_num > 0{
            initial_ascending = false;
        }
    }else{ // If the array is empty or doesn't have at least 2 numbers gtfo it aint safe fam
        return false;
    }

    // Main loop to slide compare numbers
    for (index, num) in report.iter().enumerate(){
        // Skip the last element if we're at the end since no point in checking and prevents edge case error
        if index == report.len()-1{
            continue;
        }

        let local_ascending = *num - report[index+1] < 0; // See if this pair is ascending or descending
        if local_ascending != initial_ascending{
            is_safe = false; // If this slope doesnt match initial, quit out
            println!("Slope changed! Initial slope: {}, new slope: {}. For pair: {}, {}", initial_ascending, local_ascending, num, report[index+1]);
            break;
        }
        if *num - report[index+1] == 0{ // If the pair's slope is 0 -- ACTUALLY THIS WILL NEVER BE RUN CAUSE I EXCLUDE THIS ABOVE IN MY CONDTIONIAL LOOK AT HOW SMART I AM HEHEHE
            is_safe = false;
            // println!("Slope is zero!");
            break;
        }

        // I don't have to dereference num here (which is given as a reference from enumerate) since i32 implements operator traits and the complier will do it for me
        if (num - report[index+1]).abs() < 1 || (num - report[index+1]).abs() > 3{
            is_safe = false;
            break;
        }
    }

    return is_safe;
}

fn main() {
    let mut safe_report_count = 0;

    let levels_of_reports = load_inputs("src/input.txt");
    //println!("{:#?}", levels_of_reports);

    for report in &levels_of_reports{
        if is_report_safe(report){ // This will proceed if true, don't need to write == true explicitly
            safe_report_count += 1;
        }
    }

    println!("Total Safe Reports is: {}", safe_report_count);
}
