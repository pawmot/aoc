#[cfg(test)]
mod day12 {
    use std::{collections::VecDeque, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day12::get_data,
    };

    fn a(data: (Vec<Vec<u8>>, (usize, usize), (usize, usize))) -> usize {
        let (height_map, start, end) = data;
        let rc = height_map.len();
        let cc = height_map[0].len();

        let mut visited = vec![vec![false; cc]; rc];
        let mut queue = VecDeque::from([(start, 0)]);

        while let Some((pos @ (r, c), steps)) = queue.pop_front() {
            if pos == end {
                return steps;
            }

            if visited[r][c] {
                continue;
            }

            visited[r][c] = true;
            let height = height_map[r][c];

            [
                ((r as i32) - 1, c as i32),
                (r as i32, (c as i32) - 1),
                ((r as i32) + 1, c as i32),
                (r as i32, (c as i32) + 1),
            ]
            .into_iter()
            .filter(|(nr, nc)| !(*nr < 0 || *nr >= rc as i32 || *nc < 0 || *nc >= cc as i32))
            .map(|(nr, nc)| (nr as usize, nc as usize))
            .filter(|(nr, nc)| height_map[*nr][*nc] <= height + 1)
            .map(|pos| (pos, steps + 1))
            .for_each(|descriptor| queue.push_back(descriptor));
        }

        panic!("Exhausted search without solution :[");
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 31);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 330);
        Ok(())
    }

    fn b(data: (Vec<Vec<u8>>, (usize, usize), (usize, usize))) -> usize {
        let (height_map, _, end) = data;
        let rc = height_map.len();
        let cc = height_map[0].len();

        let mut visited = vec![vec![false; cc]; rc];
        let mut queue = VecDeque::from([(end, 0)]);

        while let Some(((r, c), steps)) = queue.pop_front() {
            let height = height_map[r][c];

            if height == 0 {
                return steps;
            }

            if visited[r][c] {
                continue;
            }

            visited[r][c] = true;

            [
                ((r as i32) - 1, c as i32),
                (r as i32, (c as i32) - 1),
                ((r as i32) + 1, c as i32),
                (r as i32, (c as i32) + 1),
            ]
            .into_iter()
            .filter(|(nr, nc)| !(*nr < 0 || *nr >= rc as i32 || *nc < 0 || *nc >= cc as i32))
            .map(|(nr, nc)| (nr as usize, nc as usize))
            .filter(|(nr, nc)| height_map[*nr][*nc] >= height - 1)
            .map(|pos| (pos, steps + 1))
            .for_each(|descriptor| queue.push_back(descriptor));
        }

        panic!("Exhausted search without solution :[");
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 29);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 321);
        Ok(())
    }
}
