use crate::service::dataTypes::{DiffChunk, DiffType, FileDiff, LineChange};
use crate::service::inputOutput::read_file_lines;

pub fn compare_files(file1: &str, file2: &str) -> Result<FileDiff, std::io::Error> {
    let lines1 = read_file_lines(file1)?; // Read lines from file1
    let lines2 = read_file_lines(file2)?; // Read lines from file2

    let m = lines1.len(); // Length of file1 (number of lines)
    let n = lines2.len(); // Length of file2 (number of lines)

    let max = m + n;
    let mut v = vec![0; 2 * max + 1]; // Vector to store the furthest points

    let mut diff = FileDiff::new(file2.to_string()); // Create a new FileDiff

    // Main loop to compute the edit script
    for d in 0..=max {
        for k in (-(d as isize)..=d as isize).step_by(2) {
            let k_offset = (k + max as isize) as usize;
            let mut x;

            if k == -(d as isize) || (k != d as isize && v[k_offset - 1] < v[k_offset + 1]) {
                x = v[k_offset + 1]; // Move right (insertion)
            } else {
                x = v[k_offset - 1] + 1; // Move down (deletion)
            }

            let mut y = (x as isize - k) as usize;

            // Follow the diagonal as far as possible (matching lines)
            while x < m && y < n && lines1[x] == lines2[y] {
                x += 1;
                y += 1;
            }

            v[k_offset] = x;

            // Check if the end of both files has been reached
            if x >= m && y >= n {
                let mut i = m;
                let mut j = n;
                let mut current_chunk = DiffChunk {
                    diff_type: DiffType::Addition, // Placeholder
                    changes: Vec::new(),
                };

                // Backtrack to generate the diff
                while i > 0 || j > 0 {
                    let k = (i as isize - j as isize) as usize;
                    let k_offset = (k as isize + max as isize) as usize;

                    if i > 0 && (j == 0 || v[k_offset - 1] < v[k_offset + 1]) {
                        i -= 1;
                        current_chunk.diff_type = DiffType::Deletion;
                        current_chunk.changes.push(LineChange {
                            line_number: i + 1,
                            content: lines1[i].clone(),
                        });
                    } else if j > 0 && (i == 0 || v[k_offset - 1] >= v[k_offset + 1]) {
                        j -= 1;
                        current_chunk.diff_type = DiffType::Addition;
                        current_chunk.changes.push(LineChange {
                            line_number: j + 1,
                            content: lines2[j].clone(),
                        });
                    } else {
                        i -= 1;
                        j -= 1;
                    }
                }

                // Add any remaining chunk to the diff
                if !current_chunk.changes.is_empty() {
                    diff.add_chunk(current_chunk);
                }

                return Ok(diff);
            }
        }
    }

    Ok(diff) // Return the computed diff
}