use crate::data::common::{read_lines, DatasetType};
use std::io;

// TODO: investigate ways to make this simpler, possibly do the collection to io::Result in read_lines
pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<String>> {
    let lines = read_lines((2015, 2), dataset_type)?;
    Ok(lines.into_iter().collect::<io::Result<Vec<_>>>()?
        .into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>())
}
