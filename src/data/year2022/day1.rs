use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Vec<u32>>> {
    let lines = read_lines((2022, 1), dataset_type)?;

    let result = lines
        .split(|l| l.is_empty())
        .filter(|s| s.len() != 0)
        .map(|s| s.into_iter().map(|l| l.parse().unwrap()).collect())
        .collect();

    Ok(result)
}
