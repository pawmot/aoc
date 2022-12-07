use crate::data::common::{read_lines, DatasetType};
use std::io;

pub enum TermOutput {
    CdDown(String),
    CdUp,
    Ls,
    Dir(String),
    File(String, u32),
    EOF,
}

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<TermOutput>> {
    let lines = read_lines((2022, 7), dataset_type)?;

    let result = lines
        .into_iter()
        .map(|l| {
            if l.is_empty() {
                TermOutput::EOF
            } else if l == "$ cd .." {
                TermOutput::CdUp
            } else if l.starts_with("$ cd") {
                TermOutput::CdDown(l.split(" ").last().unwrap().to_string())
            } else if l == "$ ls" {
                TermOutput::Ls
            } else if l.starts_with("dir") {
                TermOutput::Dir(l.split(" ").last().unwrap().to_string())
            } else {
                let (size_str, name) = l.split_once(" ").unwrap();
                let size = size_str.parse().unwrap();
                TermOutput::File(name.to_string(), size)
            }
        })
        .collect();

    Ok(result)
}
