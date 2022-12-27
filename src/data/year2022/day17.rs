use anyhow::Result;

use crate::data::common::{read_lines, DatasetType};

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<char>> {
    let lines = read_lines((2022, 17), dataset_type)?;
    Ok(lines[0].chars().collect())
}
