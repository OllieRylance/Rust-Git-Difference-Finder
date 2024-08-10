use crate::read_file_lines;


#[derive(Debug, PartialEq)]
enum DiffType {
    Addition,
    Deletion,
    Modification,
}

#[derive(Debug)]
struct LineChange {
    line_number: usize,
    content: String,
}

#[derive(Debug)]
struct DiffChunk {
    diff_type: DiffType,
    old_line_number: Option<usize>, // None if the change is an addition
    new_line_number: Option<usize>, // None if the change is a deletion
    changes: Vec<LineChange>,
}

#[derive(Debug)]
struct FileDiff {
    file_name: String,
    chunks: Vec<DiffChunk>,
}

impl FileDiff {
    fn new(file_name: String) -> Self {
        FileDiff {
            file_name,
            chunks: Vec::new(),
        }
    }

    fn add_chunk(&mut self, chunk: DiffChunk) {
        self.chunks.push(chunk);
    }
}


// Function to compare two files line by line using the naive algorithm
pub fn compare_files(file1: &str, file2: &str) -> Result<(), std::io::Error> {
    // Implement your naive line-by-line comparison here
    // For simplicity, let's just print the file names
    println!("Comparing {} and {} using the naive algorithm", file1, file2);

    // The naive algorithm is simple: read the files line by line and compare them
    let lines1 = read_file_lines(file1)?;
    let lines2 = read_file_lines(file2)?;

    // If the files have different numbers of lines, add empty lines to the shorter file to make them the same length
    let max_lines = lines1.len().max(lines2.len());
    let lines1: Vec<String> = lines1.iter().chain(std::iter::repeat(&String::new())).take(max_lines).cloned().collect();
    let lines2: Vec<String> = lines2.iter().chain(std::iter::repeat(&String::new())).take(max_lines).cloned().collect();

    // Compare the lines and store the differences in a vector
    let mut differences = Vec::new();
    for (i, (line1, line2)) in lines1.iter().zip(lines2.iter()).enumerate() {
        if line1 != line2 {
            differences.push((i, line1.clone(), line2.clone()));
        }
    }

    // Print the differences
    print_differences(&lines2, &differences);

    Ok(())
}

// Function to print the differences between two files
fn print_differences(final_lines: &Vec<String>, differences: &Vec<(usize, String, String)>) {
    // Print the differences
    // Do this in the format of:
    //   If there is no difference, print the line number and the line from the second file
    //   If there is a difference "- Line number: file1 line\n+ Line number: file1 line"
    // I would like to print the differences in the following format:
    //   Line before difference, difference, line after difference
    // If there are multiple differences in a row, print them all with the files together

    // Create a variable to store the current chain of differences
    let mut current_chain = Vec::new();

    for (current_line_index, current_line) in final_lines.iter().enumerate() {
        // If the current line is in the differences vector, add it to the current chain
        if differences.iter().any(|(difference_line_index, _, _)| *difference_line_index == current_line_index) {
            current_chain.push(current_line_index);
        } // Else if current chain is not empty, print the line before the chain, the current chain, the current line, and clear the chain
        else if !current_chain.is_empty()
        {
            // Print information about the differences currently in the chain
            // This should take the format "Differences found from line i to line j" if the chain is [i, i+1, ..., j]
            // Otherwise, print "Differences found on line i" if the chain is [i]
            if current_chain.len() > 1 {
                let start_line_number = current_chain[0] + 1;
                let end_line_number = current_chain[current_chain.len() - 1] + 1;
                println!("Differences found from line {} to line {}", start_line_number, end_line_number);
            } else {
                let line_number = current_chain[0] + 1;
                println!("Differences found on line {}", line_number);
            }

            // Get the maximum line index to pad the line numbers
            let max_line_index = current_chain.iter().max().unwrap();

            // Print the line before the chain
            let line_index_before_chain = current_line_index - current_chain.len() - 1;
            if line_index_before_chain >= 0 {
                let padded_line_number = pad_line_number(line_index_before_chain, *max_line_index);
                println!("  {} | {}", padded_line_number, final_lines[line_index_before_chain]);
            }

            // Print the chain of differences
            // First print the first file's lines in red
            for difference_line_index in &current_chain {
                let padded_line_number = pad_line_number(*difference_line_index, *max_line_index);

                // Get the line from differences where the line number is equal to j
                let (_, line1, _) = differences.iter().find(|(line_number, _, _)| *line_number == *difference_line_index).unwrap();

                println!("\x1b[31m- {} | {}\x1b[0m", padded_line_number, line1);
            }
            // Then print the second file's lines in green
            for difference_line_index in &current_chain {
                let padded_line_number = pad_line_number(*difference_line_index, *max_line_index);

                // Get the line from differences where the line number is equal to j
                let (_, _, line2) = differences.iter().find(|(line_number, _, _)| *line_number == *difference_line_index).unwrap();

                println!("\x1b[32m+ {} | {}\x1b[0m", padded_line_number, line2);
            }

            // Print the current line
            let padded_line_number = pad_line_number(current_line_index, *max_line_index);
            println!("  {} | {}", padded_line_number, current_line);

            // Print a newline to separate the output of different file comparisons
            println!();

            // Clear the chain
            current_chain.clear();
        }
    }
}


// Function to take a line index and the maximum line index and return a string with the line number padded to the length of the maximum line number
fn pad_line_number(line_index: usize, max_line_index: usize) -> String {
    let line_number = line_index + 1;
    let line_number_string = line_number.to_string();
    let max_line_number = max_line_index + 1;
    let max_line_number_string = max_line_number.to_string();
    let padding = max_line_number_string.len() - line_number_string.len();
    let padding_string = " ".repeat(padding);
    format!("{}{}", padding_string, line_number)
}