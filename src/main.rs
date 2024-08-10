mod algorithms;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define paths to the test files and whether to use them
    let base_path = "src/test_files/";
    let englishFile1 = format!("{}englishFile1.txt", base_path);
    let englishFile2 = format!("{}englishFile2.txt", base_path);
    let useEnglish = true;
    let pythonFile1 = format!("{}pythonFile1.py", base_path);
    let pythonFile2 = format!("{}pythonFile2.py", base_path);
    let usePython = true;

    // Store the file paths and whether to use them in a vector
    let files = vec![
        (englishFile1, englishFile2, useEnglish),
        (pythonFile1, pythonFile2, usePython),
    ];

    // Ask the user which algorithm to use
    let mut algorithmToUse = String::new();

    loop {
        println!("Which algorithm would you like to use?");
        println!("0: Naive");
        println!("1: LCS");
        println!("2: Myers");
        println!("3: Patience");
        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        // Check if the input is valid. If it is, break the loop
        algorithmToUse = input.trim().to_string();
        if algorithmToUse == "0" || algorithmToUse == "1" || algorithmToUse == "2" || algorithmToUse == "3" {
            break;
        }
        println!("Invalid algorithm");
    }

    // Go through files and if they are to be used, compare them
    for (file1, file2, useFile) in files {
        if useFile {
            match algorithmToUse.as_str() {
                "0" => algorithms::naive::compare_files(&file1, &file2)?,
                "1" => algorithms::lcs::compare_files(&file1, &file2)?,
                "2" => algorithms::myers::compare_files(&file1, &file2)?,
                "3" => algorithms::patience::compare_files(&file1, &file2)?,
                _ => println!("Invalid algorithm"),
            }
            // Print a newline to separate the output of different file comparisons
            println!();
        }
    }

    Ok(())
}
