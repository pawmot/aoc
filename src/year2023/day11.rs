#[cfg(test)]
mod day11 {
    use std::cmp::{min, max};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day11::get_data,
    };
    use anyhow::Result;

    fn a(data: Vec<Vec<char>>) -> usize {
        let empty_rows = data
            .iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|c| *c == '.'))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let empty_cols = {
            let mut empty_cols = vec![];
            for col in 0..data[0].len() {
                if data.iter().all(|r| r[col] == '.') {
                    empty_cols.push(col);
                }
            }
            empty_cols
        };

        let mut galaxies = vec![];

        for r in 0..data.len() {
            for c in 0..data[r].len() {
                if data[r][c] == '#' {
                    galaxies.push((r, c));
                }
            }
        }

        galaxies.sort_by_key(|(_, c)| *c);
        galaxies.sort_by_key(|(r, _)| *r);

        let mut sum = 0;

        for g1 in 0..galaxies.len() {
            for g2 in g1 + 1..galaxies.len() {
                let (r1, c1) = galaxies[g1];
                let (r2, c2) = galaxies[g2];

                let min_c = min(c1, c2);
                let max_c = max(c1, c2);

                let empty_rows_between = empty_rows.iter().filter(|r| r1 < **r && **r < r2).count();
                let empty_cols_between = empty_cols.iter().filter(|c| min_c < **c && **c < max_c).count();

                sum += (r2 - r1) + (max_c - min_c) + empty_cols_between + empty_rows_between;
            }
        }

        sum
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 374);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 9233514);
        Ok(())
    }

    fn b(data: Vec<Vec<char>>, expansion_coeff: usize) -> usize {
        let empty_rows = data
            .iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|c| *c == '.'))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let empty_cols = {
            let mut empty_cols = vec![];
            for col in 0..data[0].len() {
                if data.iter().all(|r| r[col] == '.') {
                    empty_cols.push(col);
                }
            }
            empty_cols
        };

        let mut galaxies = vec![];

        for r in 0..data.len() {
            for c in 0..data[r].len() {
                if data[r][c] == '#' {
                    galaxies.push((r, c));
                }
            }
        }

        galaxies.sort_by_key(|(_, c)| *c);
        galaxies.sort_by_key(|(r, _)| *r);

        let mut sum = 0;

        for g1 in 0..galaxies.len() {
            for g2 in g1 + 1..galaxies.len() {
                let (r1, c1) = galaxies[g1];
                let (r2, c2) = galaxies[g2];

                let min_c = min(c1, c2);
                let max_c = max(c1, c2);

                let empty_rows_between = empty_rows.iter().filter(|r| r1 < **r && **r < r2).count();
                let empty_cols_between = empty_cols.iter().filter(|c| min_c < **c && **c < max_c).count();

                sum += (r2 - r1) + (max_c - min_c) + (expansion_coeff - 1) * (empty_cols_between + empty_rows_between);
            }
        }

        sum
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data, 100), 8410);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data, 1000000), 363293506944);
        Ok(())
    }
}