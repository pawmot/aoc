use std::io;
use crate::data::common::{DatasetType, read_lines};

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Vec<u32>>> {
  let lines = read_lines((2022, 1), dataset_type)?;

  let mut result = vec![vec![]];

  for line_res in lines {
    let line = line_res?;

    if line.is_empty() {
      result.push(vec![]);
    } else {
      let parsed: u32 = line.parse().unwrap();
      result.last_mut().unwrap().push(parsed);
    }
  }

  //remove the empty vector added because the newline at the end of data file
  result.pop();
  return Ok(result)
}

