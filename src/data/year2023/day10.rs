use anyhow::Result;

use crate::data::common::{read_lines, DatasetType};

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Vec<char>>> {
    let lines = read_lines((2023, 10), dataset_type)?;
    Ok(lines.into_iter().filter(|l| !l.is_empty()).map(|l| l.chars().collect()).collect())
}
