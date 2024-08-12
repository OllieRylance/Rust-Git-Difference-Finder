use crate::service::dataTypes::{DiffChunk, DiffType, FileDiff, LineChange};
use crate::service::inputOutput::read_file_lines;

pub fn compare_files(file1: &str, file2: &str) -> Result<FileDiff, std::io::Error> {
    // Read the lines of the first file into a vector of strings
    let lines1 = read_file_lines(file1)?;
    // Read the lines of the second file into a vector of strings
    let lines2 = read_file_lines(file2)?;

    // Get the number of lines in the first file
    let m = lines1.len();
    // Get the number of lines in the second file
    let n = lines2.len();

    // The maximum number of steps needed in the edit graph (sum of lengths of both files)
    let max = m + n;
    // Initialize a vector to track the furthest x-coordinate for each diagonal
    let mut v = vec![0; 2 * max + 1];

    // To track the best path, we maintain the best diff and its length
    let mut best_diff = None;
    let mut best_length = usize::MAX;

    // Main loop to compute the minimal edit script using Myers' algorithm
    for d in 0..=max {
        // Loop over all possible diagonals from -d to +d in steps of 2
        for k in (-(d as isize)..=d as isize).step_by(2) {
            // Calculate the offset for the current diagonal in the v array
            let k_offset = (k + max as isize) as usize;
            let mut x;

            // Determine the direction to move in the edit graph
            if k == -(d as isize) || (k != d as isize && v[k_offset - 1] < v[k_offset + 1]) {
                // Move right (an insertion in file2)
                x = v[k_offset + 1];
            } else {
                // Move down (a deletion in file1)
                x = v[k_offset - 1] + 1;
            }

            // Move diagonally as long as lines are the same
            let mut y = (x as isize - k) as usize;
            while x < m && y < n && lines1[x] == lines2[y] {
                x += 1;
                y += 1;
            }

            // Update the furthest x-coordinate reached for this diagonal
            v[k_offset] = x;

            // If we have reached the end of both files, we can backtrack to create the diff
            if x >= m && y >= n {
                // Start backtracking from the end of the files
                let mut i = m;
                let mut j = n;
                let mut current_diff = FileDiff::new(file2.to_string());

                // Temporary chunk to hold changes
                let mut current_chunk = DiffChunk {
                    diff_type: DiffType::Addition, // Placeholder, will adjust based on the operation
                    changes: Vec::new(),
                };

                // Display the edit graph for debugging
                println!("Edit graph for files {} and {}:", file1, file2);
                for i in 0..=max {
                    for j in 0..=max {
                        if i == 0 && j == 0 {
                            print!("  ");
                        } else if i == 0 {
                            print!("{:2} ", j);
                        } else if j == 0 {
                            print!("{:2} ", i);
                        } else {
                            let k = (i as isize - j as isize) as usize;
                            let k_offset = (k as isize + max as isize) as usize;
                            if v[k_offset] >= i {
                                print!("\\ ");
                            } else {
                                print!("| ");
                            }
                        }
                    }
                    println!();
                }

                // Backtrack through the edit graph to generate the sequence of changes
                while i > 0 || j > 0 {
                    // Calculate the current diagonal (k) based on the line numbers
                    let k = (i as isize - j as isize) as usize;
                    // Calculate the offset for the current diagonal in the v array
                    let k_offset = (k as isize + max as isize) as usize;

                    println!("Backtracking: i = {}, j = {}, k = {}, k_offset = {}", i, j, k, k_offset);

                    if i > 0 && (j == 0 || v[k_offset - 1] < v[k_offset + 1]) {
                        // If we're moving down, it means a line from file1 was deleted
                        i -= 1;

                        println!("Line deleted from file1: {}", lines1[i]);

                        // Start a new chunk if the current one is not empty
                        if !current_chunk.changes.is_empty() {
                            current_diff.add_chunk(current_chunk);
                            current_chunk = DiffChunk {
                                diff_type: DiffType::Deletion,
                                changes: Vec::new(),
                            };
                        }
                        current_chunk.changes.push(LineChange {
                            line_number: i + 1, // 1-based line number for consistency
                            content: lines1[i].clone(),
                        });
                    } else if j > 0 && (i == 0 || v[k_offset - 1] >= v[k_offset + 1]) {
                        // If we're moving right, it means a line from file2 was added
                        j -= 1;

                        println!("Line added to file2: {}", lines2[j]);

                        // Start a new chunk if the current one is not empty
                        if !current_chunk.changes.is_empty() {
                            current_diff.add_chunk(current_chunk);
                            current_chunk = DiffChunk {
                                diff_type: DiffType::Addition,
                                changes: Vec::new(),
                            };
                        }
                        current_chunk.changes.push(LineChange {
                            line_number: j + 1, // 1-based line number for consistency
                            content: lines2[j].clone(),
                        });
                    } else {
                        // If we're moving diagonally, it means the lines match and we move on
                        println!("Lines match: file1[{}] = {}, file2[{}] = {}", i - 1, lines1[i - 1], j - 1, lines2[j - 1]);
                        i -= 1;
                        j -= 1;
                    }
                }

                // Add the last chunk if it contains any changes
                if !current_chunk.changes.is_empty() {
                    current_diff.add_chunk(current_chunk);
                }

                // If the current diff is shorter than the best one found so far, update best_diff
                if current_diff.chunks.len() < best_length {
                    best_length = current_diff.chunks.len();
                    best_diff = Some(current_diff);
                }
            }
        }
    }

    // Return the best diff found
    best_diff.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No diff found"))
}
