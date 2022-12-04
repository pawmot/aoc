#[cfg(test)]
mod day3 {
    use std::{collections::HashSet, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day3::get_data,
    };

    fn a(data: Vec<String>) -> u32 {
        data.into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .map(|v| {
                let split = v.split_at(v.len() / 2);
                (split.0.to_vec(), split.1.to_vec())
            })
            .flat_map(|(l, r)| {
                let mut left_set = HashSet::new();
                let mut right_set = HashSet::new();
                l.into_iter().for_each(|c| drop(left_set.insert(c)));
                r.into_iter().for_each(|c| drop(right_set.insert(c)));
                left_set
                    .intersection(&right_set)
                    .copied()
                    .collect::<Vec<_>>()
            })
            .map(|c| {
                if c.is_ascii_lowercase() {
                    (c as u32) - 96
                } else {
                    (c as u32) - 38
                }
            })
            .sum()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 157);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 8493);
        Ok(())
    }

    fn b(data: Vec<String>) -> u32 {
        data.chunks(3)
            .into_iter()
            .map(|ch| {
                ch.into_iter()
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .collect::<Vec<_>>()
            })
            .map(|sets| {
                sets.into_iter()
                    .reduce(|acc, next| acc.intersection(&next).copied().collect::<HashSet<_>>())
                    .unwrap()
            })
            .map(|set| set.into_iter().next().unwrap())
            .map(|auth| {
                if auth.is_ascii_lowercase() {
                    (auth as u32) - 96
                } else {
                    (auth as u32) - 38
                }
            })
            .sum()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 70);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 2552);
        Ok(())
    }
}
