#[cfg(test)]
mod day5 {
    use multimap::MultiMap;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2015::day5::get_data,
    };
    use std::{collections::HashSet, io};

    fn a(data: Vec<String>) -> usize {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        data.into_iter()
            .filter(|s| {
                let mut vowels_count = 0;
                let mut double_present = false;
                let mut last_char: Option<char> = None;

                for c in s.chars() {
                    if vowels.contains(&c) {
                        vowels_count += 1;
                    }

                    if let Some(last) = last_char {
                        if last == c {
                            double_present = true;
                        }

                        match (last, c) {
                            ('a', 'b') => return false,
                            ('c', 'd') => return false,
                            ('p', 'q') => return false,
                            ('x', 'y') => return false,
                            _ => (),
                        }
                    }

                    last_char = Some(c);
                }

                vowels_count >= 3 && double_present
            })
            .count()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample0")))?;

        assert_eq!(a(data), 2);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 255);
        Ok(())
    }

    fn b(data: Vec<String>) -> usize {
        data.into_iter()
            .filter(|s| {
                let mut pairs = MultiMap::new();
                let mut duplicate_exists = false;
                let mut duplicate_pair_exists = false;
                let mut prevprev_char: Option<char> = None;
                let mut prev_char: Option<char> = None;

                for (idx,c) in s.chars().enumerate() {
                    if let Some(last) = prev_char {
                        let pair = format!("{}{}", last, c);
                        if pairs.contains_key(&pair) {
                            if pairs.get_vec(&pair)
                                .unwrap()
                                .iter()
                                .copied()
                                .any(|i| idx - i > 1) {
                                duplicate_pair_exists = true;
                            }
                        }
                        pairs.insert(pair, idx);
                    }

                    if let Some(prevprev) = prevprev_char {
                        if prevprev == c {
                            duplicate_exists = true;
                        }
                    }

                    prevprev_char = prev_char;
                    prev_char = Some(c);
                }

                duplicate_exists && duplicate_pair_exists
            })
            .count()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(b(data), 2);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 55);
        Ok(())
    }
}
