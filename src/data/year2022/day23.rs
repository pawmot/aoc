use std::collections::HashSet;

use anyhow::Result;

use crate::data::common::{read_lines, DatasetType};

pub fn get_data(dataset_type: DatasetType) -> Result<HashSet<(i64, i64)>> {
    let lines = read_lines((2022, 23), dataset_type)?;
    let chars: Vec<Vec<char>> = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    Ok(chars
        .into_iter()
        .enumerate()
        .flat_map(|(ri, r)| {
            r.into_iter()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(ci, _)| (ri as i64, ci as i64))
        })
        .collect())
}
