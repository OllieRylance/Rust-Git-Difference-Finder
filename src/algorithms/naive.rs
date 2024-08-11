use crate::service::inputOutput::read_file_lines;
use crate::service::dataTypes::{DiffChunk, DiffType, FileDiff, LineChange};

// Function to compare two files line by line using the naive algorithm
pub fn compare_files(file1: &str, file2: &str) -> Result<FileDiff, std::io::Error> {
    let lines1 = read_file_lines(file1)?;
    let lines2 = read_file_lines(file2)?;

    let max_lines = lines1.len().max(lines2.len());

    let mut file_diff = FileDiff::new(file2.to_string());

    // For line_index in max_lines
    for line_index in 0..max_lines {
        // Get the line from lines1 and lines2. If line1 doesn't exist, add an addition chunk. If line2 doesn't exist, add a deletion chunk
        if line_index >= lines1.len() {
            let diff_type = DiffType::Addition;

            let changes = vec![
                LineChange {
                    line_number: line_index + 1,
                    content: lines2[line_index].clone(),
                }
            ];

            let chunk = DiffChunk {
                diff_type,
                // old_line_number: None,
                // new_line_number: Some(line_index + 1),
                changes,
            };

            file_diff.add_chunk(chunk);
            continue;
        }
        else if line_index >= lines2.len() {
            let diff_type = DiffType::Deletion;

            let changes = vec![
                LineChange {
                    line_number: line_index + 1,
                    content: lines1[line_index].clone(),
                }
            ];

            let chunk = DiffChunk {
                diff_type,
                // old_line_number: Some(line_index + 1),
                // new_line_number: None,
                changes,
            };

            file_diff.add_chunk(chunk);
            continue;
        }

        // Get the lines from lines1 and lines2
        let line1 = &lines1[line_index];
        let line2 = &lines2[line_index];

        if line1 != line2 {
            let diff_type = DiffType::Modification;

            let changes = vec![
                LineChange {
                    line_number: line_index + 1,
                    content: line1.clone(),
                },
                LineChange {
                    line_number: line_index + 1,
                    content: line2.clone(),
                },
            ];

            let chunk = DiffChunk {
                diff_type,
                // old_line_number: Some(line_index + 1),
                // new_line_number: Some(line_index + 1),
                changes,
            };

            file_diff.add_chunk(chunk);
        }
    }

    Ok(file_diff)
}