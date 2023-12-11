#[cfg(test)]
mod day9 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day9::get_data,
    };
    use anyhow::Result;

    fn get_next_element(data: Vec<i32>) -> i32 {
        let first = data[0];
        if data.iter().all(|n| *n == first) {
            first
        } else {
            let diffs = data.windows(2).map(|w| w[1] - w[0]).collect();
            let next_diff = get_next_element(diffs);
            data[data.len() - 1] + next_diff
        }
    }

    fn a(data: Vec<Vec<i32>>) -> i32 {
        data.into_iter().map(get_next_element).sum()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 114);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1684566095);
        Ok(())
    }

    fn b(data: Vec<Vec<i32>>) -> i32 {
        data.into_iter().map(get_prev_element).sum()
    }

    fn get_prev_element(data: Vec<i32>) -> i32 {
        let first = data[0];
        if data.iter().all(|n| *n == first) {
            first
        } else {
            let diffs = data.windows(2).map(|w| w[1] - w[0]).collect();
            let prev_diff = get_prev_element(diffs);
            first - prev_diff
        }
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 2);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 1136);
        Ok(())
    }
}