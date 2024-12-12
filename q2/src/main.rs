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

fn is_report_safe(report: Vec<i32>) -> bool {

    return true;
}

fn main() {
    let levels_of_reports = load_inputs("src/input.txt");
    println!("{:#?}", levels_of_reports);
}
