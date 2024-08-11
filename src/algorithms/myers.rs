use crate::service::dataTypes::{DiffChunk, DiffType, FileDiff, LineChange};
use crate::service::inputOutput::read_file_lines;

pub fn compare_files(file1: &str, file2: &str) -> Result<FileDiff, std::io::Error> {
    let lines1 = read_file_lines(file1)?;
    let lines2 = read_file_lines(file2)?;

    let m = lines1.len();
    let n = lines2.len();

    let max = m + n;
    let mut v = vec![0; 2 * max + 1];

    let mut diff = FileDiff::new(file2.to_string());

    for d in 0..=max {
        for k in (-(d as isize)..=d as isize).step_by(2) {
            let k_offset = (k + max as isize) as usize;

            let mut x;
            if k == -(d as isize) || (k != d as isize && v[k_offset - 1] < v[k_offset + 1]) {
                x = v[k_offset + 1];
            } else {
                x = v[k_offset - 1] + 1;
            }
            let mut y = x as isize - k;

            while x < m && (y as usize) < n && lines1[x] == lines2[y as usize] {
                x += 1;
                y += 1;
            }

            v[k_offset] = x;

            if x >= m && (y as usize) >= n {
                let mut current_chunk = DiffChunk {
                    diff_type: DiffType::Modification,
                    changes: Vec::new(),
                };

                let mut i = m as isize;
                let mut j = n as isize;
                while i > 0 || j > 0 {
                    let k = i - j;
                    let k_offset = (k + max as isize) as usize;

                    if i > 0 && (j == 0 || v[k_offset - 1] < v[k_offset + 1]) {
                        i -= 1;
                        current_chunk.diff_type = DiffType::Deletion;
                        current_chunk.changes.push(LineChange {
                            line_number: i as usize,
                            content: lines1[i as usize].clone(),
                        });
                    } else if j > 0 && (i == 0 || v[k_offset - 1] >= v[k_offset + 1]) {
                        j -= 1;
                        current_chunk.diff_type = DiffType::Addition;
                        current_chunk.changes.push(LineChange {
                            line_number: j as usize,
                            content: lines2[j as usize].clone(),
                        });
                    } else {
                        i -= 1;
                        j -= 1;
                    }
                }

                if !current_chunk.changes.is_empty() {
                    diff.add_chunk(current_chunk);
                }

                return Ok(diff);
            }
        }
    }

    Ok(diff)
}
