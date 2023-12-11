#[cfg(test)]
mod day2 {
    use std::cmp::max;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day2::{get_data, Game},
    };
    use anyhow::Result;

    fn a(data: Vec<Game>) -> u32 {
        data.into_iter()
            .filter(|g| {
                g.draws
                    .iter()
                    .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
            })
            .map(|g| g.number as u32)
            .sum()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 8);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1867);
        Ok(())
    }

    fn b(data: Vec<Game>) -> u32 {
        data.into_iter()
            .map(|g| {
                let mins = g
                    .draws
                    .into_iter()
                    .fold((0u8, 0u8, 0u8), |(ar, ag, ab), (r, g, b)| {
                        (max(r, ar), max(g, ag), max(b, ab))
                    });

                mins.0 as u32 * mins.1 as u32 * mins.2 as u32
            })
            .sum()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(b(data), 2286);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 84538);
        Ok(())
    }
}