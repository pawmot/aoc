use crate::data::common::{read_lines, DatasetType};
use std::io;

pub fn get_data(dataset_type: DatasetType) -> io::Result<(Vec<Vec<char>>, Vec<(u8, u8, u8)>)> {
    let mut lines = read_lines((2022, 5), dataset_type)?;
    lines.pop();

    let mut parts = lines.split(|l| l.is_empty()).map(|p| p.to_vec()).collect::<Vec<_>>();
    let moves_part = parts.pop().unwrap();
    let mut stack_part = parts.pop().unwrap();

    let stack_descriptor = stack_part.pop().unwrap();
    let max_stack_number = stack_descriptor.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .max().unwrap();

    let mut stacks = vec![vec![]; max_stack_number as usize];

    stack_part.reverse();
    for l in stack_part {
        for (i, c) in l.chars().enumerate() {
            if i != 0 && (i-1) % 4 == 0 && c != ' ' {
                stacks[(i-1) / 4].push(c)
            }
        }
    }

    let moves = moves_part.into_iter()
        .map(|l| l.split(" ").filter_map(|w| w.parse::<u8>().ok()).collect::<Vec<_>>())
        .map(|v| (v[0], v[1], v[2]))
        .collect();

    Ok((stacks, moves))
}
