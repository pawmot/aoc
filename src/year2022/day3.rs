#[cfg(test)]
mod day3 {
    use std::{collections::{HashMap, HashSet}, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day3::get_data,
    };

    fn a(data: Vec<String>) -> u32 {
        data.into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .map(|v| {
                let split = v.split_at(v.len()/2);
                (split.0.to_vec(), split.1.to_vec())
            })
            .flat_map(|(l, r)| {
                let mut left_set = HashSet::new();
                let mut right_set = HashSet::new();
                l.iter().for_each(|c| drop(left_set.insert(*c)));
                r.iter().for_each(|c| drop(right_set.insert(*c)));
                left_set.intersection(&right_set).map(|c| *c).collect::<Vec<_>>()
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
        let mut it = data.into_iter();
        let mut result = 0u32;

        // find a more elegant way to group items into groups of n elements
        loop {
            let next = it.next();

            if next.is_none() {
                break;
            }

            let one = next.unwrap();
            let two = it.next().unwrap();
            let three = it.next().unwrap();

            let one_set = one.chars().collect::<HashSet<_>>();
            let two_set = two.chars().collect::<HashSet<_>>();
            let three_set = three.chars().collect::<HashSet<_>>();

            let one_two_set = one_set.intersection(&two_set).map(|c| *c).collect::<HashSet<_>>();
            let auth = one_two_set.intersection(&three_set).map(|c| *c).collect::<Vec<_>>()[0];

            result += if auth.is_ascii_lowercase() {
                (auth as u32) - 96
            } else {
                (auth as u32) - 38
            }
        }

        return result;
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
