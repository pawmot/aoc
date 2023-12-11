#[cfg(test)]
mod day3 {
    use std::collections::HashSet;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day3::get_data,
    };
    use anyhow::Result;

    type Point<T> = (T, T);

    fn a(data: Vec<Vec<char>>) -> u32 {
        let mut sum = 0;
        let mut accounted_for: HashSet<Point<usize>> = HashSet::new();

        for row in 0..data.len() {
            for col in 0..data[row].len() {
                if !data[row][col].is_digit(10) && data[row][col] != '.' {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }

                            let n = (col as i32 + dx, row as i32 + dy);
                            let res = parse(&data, n);

                            if let Some((number, start)) = res {
                                if accounted_for.insert(start) {
                                    sum += number;
                                }
                            }
                        }
                    }
                }
            }
        }

        sum
    }

    fn parse(data: &Vec<Vec<char>>, start: Point<i32>) -> Option<(u32, Point<usize>)> {
        let (start_col, start_row) = start;

        if start_row < 0 || start_row as usize > data.len() || start_col < 0 || start_col as usize > data[start_row as usize].len() || !data[start_row as usize][start_col as usize].is_digit(10) {
            None
        } else {
            let mut col = start_col as usize;
            let row = start_row as usize;

            // search for the number beginning
            while col > 0 && data[row][col-1].is_digit(10) {
                col -= 1;
            }
            let beginning_col = col;

            let mut number = 0;

            // read the number
            while col < data[row].len() && data[row][col].is_digit(10) {
                number = number * 10 + data[row][col].to_digit(10).unwrap();
                col += 1;
            }

            Some((number, (beginning_col, row)))
        }
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 4361);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 539637);
        Ok(())
    }

    fn b(data: Vec<Vec<char>>) -> u32 {
        let mut sum = 0;

        for row in 0..data.len() {
            for col in 0..data[row].len() {
                if data[row][col] == '*' {
                    let mut nums: HashSet<(u32, Point<usize>)> = HashSet::new();
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }

                            let n = (col as i32 + dx, row as i32 + dy);
                            let res = parse(&data, n);

                            if let Some((number, start)) = res {
                                nums.insert((number, start));
                            }
                        }
                    }
                    if nums.len() == 2 {
                        sum += nums.into_iter().map(|(n, _)| n).reduce(|a, n| a * n).unwrap();
                    }
                }
            }
        }

        sum
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 467835);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 82818007);
        Ok(())
    }
}
