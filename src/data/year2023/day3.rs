use crate::data::common::{read_lines, DatasetType};
use anyhow::Result;

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Vec<char>>> {
    let lines = read_lines((2023, 3), dataset_type)?;

    Ok(lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|s| s.chars().collect())
        .collect())
}