use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<String> {
    let mut lines = read_lines((2015, 4), dataset_type)?;
    Ok(lines.swap_remove(0))
}
