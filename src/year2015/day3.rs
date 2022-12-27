#[cfg(test)]
mod day3 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2015::day3::get_data,
    };
    use std::{collections::HashSet, io};

    fn a(data: Vec<char>) -> usize {
        let start = (0, 0);
        let mut positions: HashSet<(i32, i32)> = data
            .into_iter()
            .scan(start, |state, n| {
                let (x, y) = *state;
                *state = match n {
                    '^' => (x, y + 1),
                    'v' => (x, y - 1),
                    '<' => (x - 1, y),
                    '>' => (x + 1, y),
                    _ => unreachable!(),
                };
                Some(*state)
            })
            .collect();
        positions.insert(start);

        positions.len()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 2);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 2572);
        Ok(())
    }

    fn b(data: Vec<char>) -> usize {
        let start = (0, 0);
        let mut positions: HashSet<(i32, i32)> = data
            .into_iter()
            .enumerate()
            .scan((start, start), |state, (idx, n)| {
                let (s1 @ (x1, y1), s2 @ (x2, y2)) = *state;
                if idx % 2 == 1 {
                    let new_s1 = match n {
                        '^' => (x1, y1 + 1),
                        'v' => (x1, y1 - 1),
                        '<' => (x1 - 1, y1),
                        '>' => (x1 + 1, y1),
                        _ => unreachable!(),
                    };
                    *state = (new_s1, s2);
                    Some((*state).0)
                } else {
                    let new_s2 = match n {
                        '^' => (x2, y2 + 1),
                        'v' => (x2, y2 - 1),
                        '<' => (x2 - 1, y2),
                        '>' => (x2 + 1, y2),
                        _ => unreachable!(),
                    };
                    *state = (s1, new_s2);
                    Some((*state).1)
                }
            })
            .collect();
        positions.insert(start);

        positions.len()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 11);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 2631);
        Ok(())
    }
}
