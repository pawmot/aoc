use crate::data::common::{read_lines, DatasetType};
use std::io;

pub enum Op {
    Noop,
    Addx(i16),
}

pub fn get_data(
    dataset_type: DatasetType,
) -> io::Result<(Vec<Vec<u8>>, (usize, usize), (usize, usize))> {
    let lines = read_lines((2022, 12), dataset_type)?;

    let mut start = (0, 0);
    let mut end = (0, 0);

    Ok((
        lines
            .into_iter()
            .enumerate()
            .filter(|(_, l)| !l.is_empty())
            .map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .map(|(c, ch)| {
                        if ch == 'S' {
                            start = (r, c);
                            'a'
                        } else if ch == 'E' {
                            end = (r, c);
                            'z'
                        } else {
                            ch
                        }
                    })
                    .map(|ch| (ch as u8) - 97)
                    .collect()
            })
            .collect(),
        start,
        end,
    ))
}
