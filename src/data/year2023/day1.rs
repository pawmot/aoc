use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<String>> {
    let lines = read_lines((2023, 1), dataset_type)?;

    Ok(lines.into_iter().filter(|line| !line.is_empty()).collect())
}