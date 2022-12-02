#[cfg(test)]
mod day1 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day1::get_data,
    };
    use std::{cmp::max, io};

    fn a(data: Vec<Vec<u32>>) -> u32 {
        data.into_iter()
            .map(|v| v.into_iter().sum())
            .reduce(|a, b| max(a, b))
            .unwrap()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 24000);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 70509);
        Ok(())
    }

    fn b(data: Vec<Vec<u32>>) -> u32 {
        let mut sums = data
            .into_iter()
            .map(|v| v.into_iter().sum())
            .collect::<Vec<u32>>();

        sums.sort_unstable();
        sums.reverse();

        return sums.into_iter().take(3).sum();
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 45000);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 208567);
        Ok(())
    }
}
