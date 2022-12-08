#[cfg(test)]
mod day8 {
    use std::{cmp::max, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day8::get_data,
    };

    fn a(data: Vec<Vec<u8>>) -> u16 {
        let mut result = 0;
        let rc = data.len();
        let cc = data[0].len();

        for ri in 0..rc {
            for ci in 0..cc {
                let height = data[ri][ci];

                let lines_of_sight: Vec<Vec<u8>> = vec![
                    (0..ri).map(|i| data[i][ci]).collect(),
                    ((ri + 1)..rc).map(|i| data[i][ci]).collect(),
                    (0..ci).map(|i| data[ri][i]).collect(),
                    ((ci + 1)..cc).map(|i| data[ri][i]).collect(),
                ];

                let visible = lines_of_sight
                    .into_iter()
                    .any(|los| los.into_iter().all(|h| h < height));

                if visible {
                    result += 1;
                }
            }
        }

        result
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 21);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1693);
        Ok(())
    }

    fn b(data: Vec<Vec<u8>>) -> u32 {
        let mut result = 0;
        let rc = data.len();
        let cc = data[0].len();

        for ri in 0..rc {
            for ci in 0..cc {
                let height = data[ri][ci];

                let lines_of_sight: Vec<Vec<u8>> = vec![
                    (0..ri).rev().map(|i| data[i][ci]).collect(),
                    ((ri + 1)..rc).map(|i| data[i][ci]).collect(),
                    (0..ci).rev().map(|i| data[ri][i]).collect(),
                    ((ci + 1)..cc).map(|i| data[ri][i]).collect(),
                ];

                let scenic_score = lines_of_sight
                    .into_iter()
                    // look for an iterator fn that is an inclusive take_while
                    .map(|los| {
                        let mut result = 0;
                        for h in los {
                            result += 1;
                            if h >= height {
                                break;
                            }
                        }
                        result
                    })
                    .product();

                result = max(result, scenic_score);
            }
        }

        result
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 8);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 422059);
        Ok(())
    }
}
