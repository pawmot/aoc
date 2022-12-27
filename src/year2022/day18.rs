#[cfg(test)]
mod day18 {
    use std::collections::{HashSet, VecDeque};

    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day18::get_data,
    };

    fn get_neighbours(
        (x, y, z): (usize, usize, usize),
        (max_x, max_y, max_z): (usize, usize, usize),
    ) -> Vec<(usize, usize, usize)> {
        let mut result = vec![];

        if x >= 1 {
            result.push((x - 1, y, z));
        }

        if x + 1 <= max_x {
            result.push((x + 1, y, z))
        }

        if y >= 1 {
            result.push((x, y - 1, z));
        }

        if y + 1 <= max_y {
            result.push((x, y + 1, z))
        }

        if z >= 1 {
            result.push((x, y, z - 1));
        }

        if z + 1 <= max_z {
            result.push((x, y, z + 1))
        }

        result
    }

    fn a(data: Vec<(usize, usize, usize)>) -> usize {
        let max_x = data.iter().map(|(x, _, _)| x).copied().max().unwrap();
        let max_y = data.iter().map(|(_, y, _)| y).copied().max().unwrap();
        let max_z = data.iter().map(|(_, _, z)| z).copied().max().unwrap();

        let mut map = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];

        for (x, y, z) in data {
            map[x][y][z] = true;
        }

        let mut result = 0;

        for x in 0..=max_x {
            for y in 0..=max_y {
                for z in 0..=max_z {
                    if !map[x][y][z] {
                        continue;
                    }
                    let neighbours_present = get_neighbours((x, y, z), (max_x, max_y, max_z))
                        .into_iter()
                        .filter(|(nx, ny, nz)| map[*nx][*ny][*nz])
                        .count();
                    result += 6 - neighbours_present;
                }
            }
        }

        result
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 64);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 4390);
        Ok(())
    }

    #[derive(Copy, Clone, PartialEq)]
    enum Block {
        Rock,
        AirUnknownReachability,
        AirReachable,
        AirUnreachable,
    }

    fn b(data: Vec<(usize, usize, usize)>) -> usize {
        use Block::*;

        let max_x = data.iter().map(|(x, _, _)| x).copied().max().unwrap();
        let max_y = data.iter().map(|(_, y, _)| y).copied().max().unwrap();
        let max_z = data.iter().map(|(_, _, z)| z).copied().max().unwrap();

        let mut map = vec![vec![vec![AirUnknownReachability; max_z + 1]; max_y + 1]; max_x + 1];

        for (x, y, z) in data {
            map[x][y][z] = Rock;
        }

        for x in 0..=max_x {
            for y in 0..=max_y {
                for z in 0..=max_z {
                    if map[x][y][z] == AirUnknownReachability {
                        let mut visited = HashSet::new();
                        let mut queue = VecDeque::from([(x, y, z)]);
                        let mut reachable = false;

                        while let Some(pos @ (cx, cy, cz)) = queue.pop_front() {
                            if visited.contains(&pos) {
                                continue;
                            }

                            visited.insert(pos);

                            if cx == 0 || cy == 0 || cz == 0 {
                                reachable = true;
                            }

                            get_neighbours(pos, (max_x, max_y, max_z))
                                .into_iter()
                                .filter(|(nx, ny, nz)| map[*nx][*ny][*nz] == AirUnknownReachability)
                                .for_each(|n| queue.push_back(n));
                        }

                        let block_state = if reachable {
                            AirReachable
                        } else {
                            AirUnreachable
                        };
                        visited
                            .into_iter()
                            .for_each(|(x, y, z)| map[x][y][z] = block_state);
                    }
                }
            }
        }

        let mut result = 0;

        for x in 0..=max_x {
            for y in 0..=max_y {
                for z in 0..=max_z {
                    if map[x][y][z] == Rock {
                        let neighbours_present = get_neighbours((x, y, z), (max_x, max_y, max_z))
                            .into_iter()
                            .filter(|(nx, ny, nz)| {
                                map[*nx][*ny][*nz] == Rock || map[*nx][*ny][*nz] == AirUnreachable
                            })
                            .count();
                        result += 6 - neighbours_present;
                    }
                }
            }
        }

        result
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 58);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 2534);
        Ok(())
    }
}
