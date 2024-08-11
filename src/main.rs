use std::io;

mod algorithms;
mod service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define paths to the test files and whether to use them
    let base_path = "src/test_files/";
    let useEnglish = true;
    let usePython = true;

    // Store the file paths and whether to use them in a vector
    let files = vec![
        (format!("{}englishFile1.txt", base_path), format!("{}englishFile2.txt", base_path), useEnglish),
        (format!("{}pythonFile1.py", base_path), format!("{}pythonFile2.py", base_path), usePython),
    ];

    // Store the algorithm to use
    let mut algorithmToUse = String::new();

    while true {
        // Ask the user which algorithm to use
        loop {
            println!("Which algorithm would you like to use?");
            println!("0: Naive");
            println!("1: LCS");
            println!("2: Myers");
            println!("3: Patience");
            println!("x: Exit");
            // Read the user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            // Check if the input is valid. If it is, break the loop
            algorithmToUse = input.trim().to_string();
            if algorithmToUse == "0" || algorithmToUse == "1" || algorithmToUse == "2" || algorithmToUse == "3" || algorithmToUse == "x" {
                break;
            }
            println!("Invalid algorithm");
        }

        // If the user wants to exit, break the loop
        if algorithmToUse == "x" {
            break;
        }

        // Go through files and if they are to be used, compare them
        for (file1, file2, useFile) in &files {
            if *useFile {
                let lines2 = service::inputOutput::read_file_lines(&file2)?;

                match algorithmToUse.as_str() {
                    "0" => service::inputOutput::print_differences(&lines2, &algorithms::naive::compare_files(&file1, &file2)?),
                    "1" => service::inputOutput::print_differences(&lines2, &algorithms::lcs::compare_files(&file1, &file2)?),
                    "2" => service::inputOutput::print_differences(&lines2, &algorithms::myers::compare_files(&file1, &file2)?),
                    "3" => algorithms::patience::compare_files(&file1, &file2)?,
                    _ => println!("Invalid algorithm"),
                }
                // Print a newline to separate the output of different file comparisons
                println!();
            }
        }
    }

    Ok(())
}
