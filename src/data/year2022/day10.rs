use crate::data::common::{read_lines, DatasetType};
use std::io;

pub enum Op {
    Noop,
    Addx(i16),
}

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Op>> {
    let lines = read_lines((2022, 10), dataset_type)?;
    Ok(lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<_>>();
            if parts[0] == "noop" {
                Op::Noop
            } else {
                Op::Addx(parts[1].parse().unwrap())
            }
        })
        .collect())
}
