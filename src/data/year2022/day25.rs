use anyhow::Result;

use crate::data::common::{read_lines, DatasetType};

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<String>> {
    let lines = read_lines((2022, 25), dataset_type)?;

    Ok(lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect())
}
