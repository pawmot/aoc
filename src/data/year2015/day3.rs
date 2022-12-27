use itertools::Itertools;

use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<char>> {
    let lines = read_lines((2015, 3), dataset_type)?;
    let chars = lines[0].chars().collect_vec();
    Ok(chars)
}
