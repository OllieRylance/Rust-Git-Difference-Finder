use std::cmp::max;
use crate::service::dataTypes::{DiffChunk, DiffType, FileDiff, LineChange};
use crate::service::inputOutput::read_file_lines;

pub fn compare_files(file1: &str, file2: &str) -> Result<FileDiff, std::io::Error> {

    // Read lines from both files into vectors of strings
    let lines1 = read_file_lines(file1)?;
    let lines2 = read_file_lines(file2)?;

    // Get the number of lines in each file
    let m = lines1.len();
    let n = lines2.len();

    // Initialize a 2D vector (dp table) for storing LCS lengths
    // The table is of size (m+1) x (n+1) with all elements initialized to 0
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill the dp table for LCS (Longest Common Subsequence)
    // We iterate over each character in the two sequences (lines1 and lines2)
    for i in 1..=m {
        for j in 1..=n {
            // If the current lines in both files match, extend the LCS by 1
            if lines1[i - 1] == lines2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                // If they don't match, take the maximum LCS length from
                // either excluding the current line in file1 or file2
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Display the dp table for debugging
    for row in &dp {
        println!("{:?}", row);
    }


    // Create a new FileDiff to store the result, using the name of file2
    let mut diff = FileDiff::new(file2.to_string());

    // Set starting indices for backtracking through the dp table
    let (mut i, mut j) = (m, n);
    // i is the index for lines1, j is the index for lines2

    // Placeholder for the current chunk of changes
    // This will hold the lines that have been added, deleted, or modified
    let mut current_chunk = DiffChunk {
        diff_type: DiffType::Modification, // Initially set to Modification
        changes: Vec::new(),
    };

    // Backtrack through the dp table to identify differences
    while i > 0 || j > 0 {
        if i == 0 {
            // If we have reached the beginning of file1, add the remaining lines in file2 as additions
            if current_chunk.diff_type != DiffType::Addition {
                // If the current chunk is not an addition, start a new addition chunk
                if !current_chunk.changes.is_empty() {
                    diff.add_chunk(current_chunk);
                }
                current_chunk = DiffChunk {
                    diff_type: DiffType::Addition,
                    changes: Vec::new(),
                };
            }
            current_chunk.changes.push(LineChange {
                line_number: j, // Line number in file2
                content: lines2[j - 1].clone(), // The added content
            });
            j -= 1;

            dp[i][j + 1] = 9;
        } else if j == 0 {
            // If we have reached the beginning of file2, add the remaining lines in file1 as deletions
            if current_chunk.diff_type != DiffType::Deletion {
                // If the current chunk is not a deletion, start a new deletion chunk
                if !current_chunk.changes.is_empty() {
                    diff.add_chunk(current_chunk);
                }
                current_chunk = DiffChunk {
                    diff_type: DiffType::Deletion,
                    changes: Vec::new(),
                };
            }
            current_chunk.changes.push(LineChange {
                line_number: i, // Line number in file1
                content: lines1[i - 1].clone(), // The deleted content
            });
            i -= 1;

            dp[i + 1][j] = 9;
        } else if dp[i][j] != max(dp[i][j-1], dp[i-1][j]) {
            // If the current lines are the optimal match in both files, it's
            // part of the LCS and not a difference. If there is a current
            // chunk of changes, finalize it and start a new chunk.
            if !current_chunk.changes.is_empty() {
                diff.add_chunk(current_chunk);
                current_chunk = DiffChunk {
                    diff_type: DiffType::Modification, // Reset to placeholder
                    changes: Vec::new(),
                };
            }
            // Move diagonally in the dp table (match in both files)
            i -= 1;
            j -= 1;

            dp[i + 1][j + 1] = 9;
        } else {
            if dp[i][j-1] < dp[i-1][j] {
                // If moving up in the dp table is optimal, it means a line was
                // deleted from file1.
                if current_chunk.diff_type != DiffType::Deletion {
                    // If the current chunk is not a deletion, start a new deletion chunk
                    if !current_chunk.changes.is_empty() {
                        diff.add_chunk(current_chunk);
                    }
                    current_chunk = DiffChunk {
                        diff_type: DiffType::Deletion,
                        changes: Vec::new(),
                    };
                }

                // If the current chunk is a deletion, add the line to the current chunk
                current_chunk.changes.push(LineChange {
                    line_number: i, // Line number in file1
                    content: lines1[i - 1].clone(), // The deleted content
                });
                i -= 1;

                dp[i + 1][j] = 9;
            } else {
                // Otherwise, we move left in the dp table, indicating an addition
                // in file2.
                if current_chunk.diff_type != DiffType::Addition {
                    // If the current chunk is not an addition, start a new addition chunk
                    if !current_chunk.changes.is_empty() {
                        diff.add_chunk(current_chunk);
                    }
                    current_chunk = DiffChunk {
                        diff_type: DiffType::Addition,
                        changes: Vec::new(),
                    };
                }

                // If the current chunk is an addition, add the line to the current chunk
                current_chunk.changes.push(LineChange {
                    line_number: j, // Line number in file2
                    content: lines2[j - 1].clone(), // The added content
                });
                j -= 1;

                // For debugging leave a mark in the dp
                dp[i][j + 1] = 9;
            }
        }

        // if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        //     // If the current lines match in both files, it's part of the LCS
        //     // and not a difference. If there is a current chunk of changes,
        //     // finalize it and start a new chunk.
        //     if !current_chunk.changes.is_empty() {
        //         diff.add_chunk(current_chunk);
        //         current_chunk = DiffChunk {
        //             diff_type: DiffType::Modification, // Reset to placeholder
        //             changes: Vec::new(),
        //         };
        //     }
        //     // Move diagonally in the dp table (match in both files)
        //     i -= 1;
        //     j -= 1;
        // } else if i > 0 && (j == 0 || dp[i - 1][j] >= dp[i][j - 1]) {
        //     // If no match and moving up in the dp table is optimal, it means
        //     // a line was deleted from file1.
        //     if current_chunk.diff_type != DiffType::Deletion {
        //         // If the current chunk is not a deletion, start a new deletion chunk
        //         if !current_chunk.changes.is_empty() {
        //             diff.add_chunk(current_chunk);
        //         }
        //         current_chunk = DiffChunk {
        //             diff_type: DiffType::Deletion,
        //             changes: Vec::new(),
        //         };
        //     }
        //
        //     // If the current chunk is a deletion, add the line to the current chunk
        //     current_chunk.changes.push(LineChange {
        //         line_number: i, // Line number in file1
        //         content: lines1[i - 1].clone(), // The deleted content
        //     });
        //     i -= 1;
        // } else if j > 0 {
        //     // Otherwise, we move left in the dp table, indicating an addition
        //     // in file2.
        //     if current_chunk.diff_type != DiffType::Addition {
        //         // If the current chunk is not an addition, start a new addition chunk
        //         if !current_chunk.changes.is_empty() {
        //             diff.add_chunk(current_chunk);
        //         }
        //         current_chunk = DiffChunk {
        //             diff_type: DiffType::Addition,
        //             changes: Vec::new(),
        //         };
        //     }
        //
        //     // If the current chunk is an addition, add the line to the current chunk
        //     current_chunk.changes.push(LineChange {
        //         line_number: j, // Line number in file2
        //         content: lines2[j - 1].clone(), // The added content
        //     });
        //     j -= 1;
        // }
    }

    // Display the dp table for debugging
    for row in &dp {
        println!("{:?}", row);
    }

    // After backtracking, if there's an unfinished chunk of changes,
    // add it to the final diff.
    if !current_chunk.changes.is_empty() {
        diff.add_chunk(current_chunk);
    }

    // Return the final diff as a result
    Ok(diff)
}
