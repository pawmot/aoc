#[cfg(test)]
mod day14 {
    use anyhow::Result;
    use std::{
        cmp::{max, min, Ordering},
        io,
    };

    use itertools::{
        EitherOrBoth::{Both, Left, Right},
        Itertools,
    };

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day14::get_data,
    };

    fn a(data: Vec<Vec<(usize, usize)>>) -> usize {
        let mut min_x = usize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for v in &data {
            for (x, y) in v {
                min_x = min(min_x, *x);
                max_x = max(max_x, *x);
                max_y = max(max_y, *y);
            }
        }

        let mut map = vec![vec![false; max_y + 1]; max_x + 1];

        for v in data {
            if v.len() == 1 {
                let (x, y) = v[0];
                map[x][y] = true;
            } else {
                v.windows(2).for_each(|segment| {
                    let (x1, y1) = segment[0];
                    let (x2, y2) = segment[1];
                    if x1 == x2 {
                        let x = x1;
                        let (y1, y2) = (min(y1, y2), max(y1, y2));
                        for y in y1..=y2 {
                            map[x][y] = true;
                        }
                    } else {
                        let y = y1;
                        let (x1, x2) = (min(x1, x2), max(x1, x2));
                        for x in x1..=x2 {
                            map[x][y] = true;
                        }
                    }
                })
            }
        }

        let mut result = 0;
        'outer: loop {
            let mut x = 500;
            let mut y = 0;
            loop {
                if x < min_x || x > max_x || y == max_y {
                    break 'outer;
                }

                let step_y = y + 1;
                if !map[x][step_y] {
                    y = step_y;
                } else if !map[x - 1][step_y] {
                    x -= 1;
                    y = step_y;
                } else if !map[x + 1][step_y] {
                    x += 1;
                    y = step_y;
                } else {
                    map[x][y] = true;
                    break;
                }
            }
            result += 1;
        }

        result
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 24);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1003);
        Ok(())
    }

    fn b(data: Vec<Vec<(usize, usize)>>) -> usize {
        let mut min_x = usize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for v in &data {
            for (x, y) in v {
                min_x = min(min_x, *x);
                max_x = max(max_x, *x);
                max_y = max(max_y, *y);
            }
        }

        let mut map = vec![vec![false; max_y + 3]; max_x + 500];

        for v in data {
            if v.len() == 1 {
                let (x, y) = v[0];
                map[x][y] = true;
            } else {
                v.windows(2).for_each(|segment| {
                    let (x1, y1) = segment[0];
                    let (x2, y2) = segment[1];
                    if x1 == x2 {
                        let x = x1;
                        let (y1, y2) = (min(y1, y2), max(y1, y2));
                        for y in y1..=y2 {
                            map[x][y] = true;
                        }
                    } else {
                        let y = y1;
                        let (x1, x2) = (min(x1, x2), max(x1, x2));
                        for x in x1..=x2 {
                            map[x][y] = true;
                        }
                    }
                })
            }
        }

        for x in 0..map.len() {
            map[x][max_y + 2] = true;
        }

        let mut result = 0;
        loop {
            let mut x = 500;
            let mut y = 0;

            if map[x][y] {
                break;
            }

            loop {
                let step_y = y + 1;
                if !map[x][step_y] {
                    y = step_y;
                } else if !map[x - 1][step_y] {
                    x -= 1;
                    y = step_y;
                } else if !map[x + 1][step_y] {
                    x += 1;
                    y = step_y;
                } else {
                    map[x][y] = true;
                    break;
                }
            }
            result += 1;
        }

        result
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 93);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 25771);
        Ok(())
    }
}
