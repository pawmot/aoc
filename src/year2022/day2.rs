#[cfg(test)]
mod day2 {
    use std::{collections::HashMap, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day2::get_data,
    };

    fn a(data: Vec<(char, char)>) -> u32 {
        let mut score_map: HashMap<(char, char), u32> = HashMap::new();
        score_map.insert(('A', 'X'), 4);
        score_map.insert(('A', 'Y'), 8);
        score_map.insert(('A', 'Z'), 3);
        score_map.insert(('B', 'X'), 1);
        score_map.insert(('B', 'Y'), 5);
        score_map.insert(('B', 'Z'), 9);
        score_map.insert(('C', 'X'), 7);
        score_map.insert(('C', 'Y'), 2);
        score_map.insert(('C', 'Z'), 6);

        data.into_iter().map(|p| score_map[&p]).sum()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 15);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 10816);
        Ok(())
    }

    fn b(data: Vec<(char, char)>) -> u32 {
        let mut score_map: HashMap<(char, char), u32> = HashMap::new();
        score_map.insert(('A', 'X'), 3);
        score_map.insert(('A', 'Y'), 4);
        score_map.insert(('A', 'Z'), 8);
        score_map.insert(('B', 'X'), 1);
        score_map.insert(('B', 'Y'), 5);
        score_map.insert(('B', 'Z'), 9);
        score_map.insert(('C', 'X'), 2);
        score_map.insert(('C', 'Y'), 6);
        score_map.insert(('C', 'Z'), 7);

        data.into_iter().map(|p| score_map[&p]).sum()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 12);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 11657);
        Ok(())
    }
}
