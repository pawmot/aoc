#[cfg(test)]
mod day2 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2015::day2::get_data,
    };
    use std::{cmp::min, io};

    fn a(data: Vec<String>) -> u32 {
        data.into_iter()
            .map(|l| {
                l.split("x")
                    .map(|d| d.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|s| (s[0] * s[1], s[1] * s[2], s[2] * s[0]))
            .map(|(s1, s2, s3)| 2 * s1 + 2 * s2 + 2 * s3 + min(s1, min(s2, s3)))
            .sum()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 101);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1588178);
        Ok(())
    }

    fn b(data: Vec<String>) -> u32 {
        data.into_iter()
            .map(|l| {
                l.split("x")
                    .map(|d| d.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|mut s| {
                s.sort_unstable();
                2 * s[0] + 2 * s[1] + s[0] * s[1] * s[2]
            })
            .sum()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 48);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 3783758);
        Ok(())
    }
}
