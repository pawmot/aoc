use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<(char, char)>> {
    let lines = read_lines((2022, 2), dataset_type)?;

    let mut result = vec![];

    for line_res in lines {
        let line = line_res?;
        if line.is_empty() {
            break;
        }
        let chars = line.split(" ")
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();

        result.push((chars[0], chars[1]));
    }

    return Ok(result);
}
