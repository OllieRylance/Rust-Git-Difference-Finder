#[derive(Debug, PartialEq)]
pub enum DiffType {
    Addition,
    Deletion,
    Modification,
}

#[derive(Debug)]
pub struct LineChange {
    pub line_number: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct DiffChunk {
    pub diff_type: DiffType,
    // old_line_number: Option<usize>, // None if the change is an addition
    // new_line_number: Option<usize>, // None if the change is a deletion
    pub changes: Vec<LineChange>,
}

#[derive(Debug)]
pub struct FileDiff {
    pub file_name: String,
    pub chunks: Vec<DiffChunk>,
}

impl FileDiff {
    pub fn new(file_name: String) -> Self {
        FileDiff {
            file_name,
            chunks: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: DiffChunk) {
        self.chunks.push(chunk);
    }
}