use std::collections::HashSet;

use anyhow::Result;

use crate::data::common::{read_lines, DatasetType};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BlizzardDirection {
    Up,
    Down,
    Left,
    Right,
}

pub type Blizzard = ((usize, usize), BlizzardDirection);

pub fn get_data(dataset_type: DatasetType) -> Result<(HashSet<Blizzard>, (usize, usize))> {
    let lines = read_lines((2022, 24), dataset_type)?;
    let h = lines.len() - 1;
    let w = lines[0].chars().count();
    let mut blizzards = HashSet::new();
    for (r, l) in lines.into_iter().enumerate() {
        for (c, ch) in l.chars().enumerate() {
            let maybe_blizzard = match ch {
                '^' => Some(((r, c), BlizzardDirection::Up)),
                'v' => Some(((r, c), BlizzardDirection::Down)),
                '>' => Some(((r, c), BlizzardDirection::Right)),
                '<' => Some(((r, c), BlizzardDirection::Left)),
                _ => None,
            };

            if maybe_blizzard.is_some() {
                blizzards.insert(maybe_blizzard.unwrap());
            }
        }
    }
    Ok((blizzards, (h, w)))
}
