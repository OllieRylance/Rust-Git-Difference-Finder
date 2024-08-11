use std::{fs, io};
use std::io::BufRead;
use std::path::Path;

// Function to read the lines of a file into a vector of strings
pub fn read_file_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}

// Function to print the differences between two files
pub fn print_differences(final_lines: &Vec<String>, file_diff: &crate::service::dataTypes::FileDiff) {
    // Print the file name in blue and bold
    println!("\x1b[1;34mFile :{}\x1b[0m", file_diff.file_name);

    // Find the line indices from the old and new files that were added, deleted, or modified
    // subtracted_lines is a dictionary where the key is the line number and the value is the content of the line
    let mut subtracted_lines = Vec::new();
    // added_lines is a vector of line numbers that were added
    let mut added_lines = Vec::new();


    for chunk in &file_diff.chunks {
        match chunk.diff_type {
            crate::service::dataTypes::DiffType::Addition => {
                for change in &chunk.changes {
                    added_lines.push(change.line_number);
                }
            }
            crate::service::dataTypes::DiffType::Deletion => {
                for change in &chunk.changes {
                    subtracted_lines.push((change.line_number, change.content.clone()));
                }
            }
            crate::service::dataTypes::DiffType::Modification => {

                for change_index in 0..chunk.changes.len() {
                    let change = &chunk.changes[change_index];
                    if change_index < chunk.changes.len() / 2 {
                        subtracted_lines.push((change.line_number, change.content.clone()));
                    } else {
                        added_lines.push(change.line_number);
                    }
                }
            }
        }
    }

    // Print all lines, marking added and deleted lines with + and -
    let mut initial_line_number = 1;
    let mut final_line_number = 1;

    while final_line_number <= final_lines.len() {
        let mut modification_displayed = false;

        // If there is a key in subtracted_lines that is equal to initial_line_number, print the line in red with a - at the beginning
        if let Some((_, line_content)) = subtracted_lines.iter().find(|(line_number, _)| *line_number == initial_line_number) {
            println!("\x1b[31m-{}\x1b[0m", line_content);
            modification_displayed = true;
            initial_line_number += 1;
        }
        // If final_line_number is in added_lines, print the line in green with a + at the beginning
        if added_lines.contains(&final_line_number) && !modification_displayed {
            println!("\x1b[32m+{}\x1b[0m", final_lines[final_line_number - 1]);
            modification_displayed = true;
            final_line_number += 1;
        }

        if !modification_displayed {
            println!(" {}", final_lines[final_line_number - 1]);
            initial_line_number += 1;
            final_line_number += 1;
        }
    }

    // Print that this is the end of the file comparison in blue and bold
    println!("\x1b[1;34mEnd of file comparison\x1b[0m");
}