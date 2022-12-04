use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<((u16, u16), (u16, u16))>> {
    let lines = read_lines((2022, 4), dataset_type)?;

    let mut result = vec![];

    for line_res in lines {
        let line = line_res?;
        if line.is_empty() {
            continue;
        }
        let parsed = line.split(",")
            .map(|p| p.split("-").map(|d| d.parse::<u16>().unwrap()))
            .map(|s| {
                let coll = s.collect::<Vec<_>>();
                (coll[0], coll[1])
            })
            .collect::<Vec<_>>();
        result.push((parsed[0], parsed[1]));
    }

    return Ok(result);
}
