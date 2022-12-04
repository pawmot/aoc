use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<((u16, u16), (u16, u16))>> {
    let lines = read_lines((2022, 4), dataset_type)?;
    let result = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(",")
                .map(|p| p.split("-").map(|d| d.parse().unwrap()).collect::<Vec<_>>())
                .map(|v| (v[0], v[1]))
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect();

    Ok(result)
}
