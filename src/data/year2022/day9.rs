use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<(char, u16)>> {
    let lines = read_lines((2022, 9), dataset_type)?;
    Ok(lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").map(|p| p.to_string()).collect::<Vec<_>>())
        .map(|v| (v[0].chars().next().unwrap(), v[1].parse().unwrap()))
        .collect())
}
