use std::io;
use crate::data::common::{DatasetType, read_lines};

pub fn get_data(dataset_type: DatasetType) -> io::Result<String> {
  let mut lines = read_lines((2015, 1), dataset_type)?;

  return lines.next().unwrap();
}

