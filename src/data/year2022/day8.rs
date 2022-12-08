use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Vec<u8>>> {
    let lines = read_lines((2022, 8), dataset_type)?;
    Ok(lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap() as u8).collect())
        .collect())
}
