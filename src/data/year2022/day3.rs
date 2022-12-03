use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<String>> {
    let lines = read_lines((2022, 3), dataset_type)?;

    let mut result = vec![];

    for line_res in lines {
        let line = line_res?;
        result.push(line);
    }

    //remove the last empty line (EOF)
    result.pop();
    return Ok(result);
}
