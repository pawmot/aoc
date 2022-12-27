#[cfg(test)]
mod day23 {
    use std::{
        cmp::{max, min},
        collections::{HashMap, HashSet},
    };

    use anyhow::Result;
    use itertools::Itertools;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day23::get_data,
    };

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Position {
        r: i64,
        c: i64,
    }

    fn get_neighbours(pos: Position) -> Vec<Position> {
        [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1].into_iter())
            .filter(|(dr, dc)| *dr != 0 || *dc != 0)
            .map(|(dr, dc)| Position {
                r: pos.r + dr,
                c: pos.c + dc,
            })
            .collect()
    }

    fn get_neighbours_north(pos: Position) -> Vec<Position> {
        [-1, 0, 1]
            .into_iter()
            .map(|dc| Position {
                r: pos.r - 1,
                c: pos.c + dc,
            })
            .collect()
    }

    fn get_neighbours_east(pos: Position) -> Vec<Position> {
        [-1, 0, 1]
            .into_iter()
            .map(|dr| Position {
                r: pos.r + dr,
                c: pos.c + 1,
            })
            .collect()
    }

    fn get_neighbours_south(pos: Position) -> Vec<Position> {
        [-1, 0, 1]
            .into_iter()
            .map(|dc| Position {
                r: pos.r + 1,
                c: pos.c + dc,
            })
            .collect()
    }

    fn get_neighbours_west(pos: Position) -> Vec<Position> {
        [-1, 0, 1]
            .into_iter()
            .map(|dr| Position {
                r: pos.r + dr,
                c: pos.c - 1,
            })
            .collect()
    }

    fn mov(pos: Position, direction: u8) -> Position {
        match direction {
            NORTH => Position {
                r: pos.r - 1,
                c: pos.c,
            },
            SOUTH => Position {
                r: pos.r + 1,
                c: pos.c,
            },
            WEST => Position {
                r: pos.r,
                c: pos.c - 1,
            },
            EAST => Position {
                r: pos.r,
                c: pos.c + 1,
            },
            _ => unreachable!(),
        }
    }

    const NORTH: u8 = 0;
    const SOUTH: u8 = 1;
    const WEST: u8 = 2;
    const EAST: u8 = 3;

    fn print_debug(positions: &HashSet<Position>) {
        let (rrange, crange) = positions.iter().fold(
            ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
            |((rmin, rmax), (cmin, cmax)), Position { r, c }| {
                (
                    (min(rmin, *r), max(rmax, *r)),
                    (min(cmin, *c), max(cmax, *c)),
                )
            },
        );

        for r in rrange.0..=rrange.1 {
            for c in crange.0..=crange.1 {
                if positions.contains(&Position { r, c }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn a(data: HashSet<(i64, i64)>) -> usize {
        let mut positions: HashSet<Position> =
            data.into_iter().map(|(r, c)| Position { r, c }).collect();

        let mut direction = NORTH;

        for _ in 0..10 {
            //print_debug(&positions);
            let mut new_positions = HashSet::new();

            let mut movement_propositions = HashMap::new();
            for pos in positions.iter().cloned() {
                let do_move = get_neighbours(pos)
                    .into_iter()
                    .any(|p| positions.contains(&p));

                if do_move {
                    let mut moved = false;
                    for n in 0..4 {
                        let dir = (direction + n) % 4;
                        let dir_neighbours = match dir {
                            NORTH => get_neighbours_north(pos),
                            SOUTH => get_neighbours_south(pos),
                            WEST => get_neighbours_west(pos),
                            EAST => get_neighbours_east(pos),
                            _ => unreachable!(),
                        };

                        let can_move =
                            !(dir_neighbours.into_iter().any(|p| positions.contains(&p)));

                        if can_move {
                            movement_propositions.insert(pos, mov(pos, dir));
                            moved = true;
                            break;
                        }
                    }
                    // not sure if necessary, rules don't seem to mention it
                    if !moved {
                        new_positions.insert(pos);
                    }
                } else {
                    new_positions.insert(pos);
                }
            }

            let mut grouped_propositions: HashMap<Position, usize> = HashMap::new();
            for (_, t) in &movement_propositions {
                grouped_propositions
                    .entry(*t)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }

            for (f, t) in movement_propositions {
                if grouped_propositions[&t] == 1 {
                    new_positions.insert(t);
                } else {
                    new_positions.insert(f);
                }
            }

            positions = new_positions;
            direction += 1;
            direction %= 4;
        }

        let (rrange, crange) = positions.iter().fold(
            ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
            |((rmin, rmax), (cmin, cmax)), Position { r, c }| {
                (
                    (min(rmin, *r), max(rmax, *r)),
                    (min(cmin, *c), max(cmax, *c)),
                )
            },
        );

        let mut result = 0;

        for r in rrange.0..=rrange.1 {
            for c in crange.0..=crange.1 {
                if !positions.contains(&Position { r, c }) {
                    result += 1;
                }
            }
        }

        result
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 110);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 4025);
        Ok(())
    }

    fn b(data: HashSet<(i64, i64)>) -> usize {
        let mut positions: HashSet<Position> =
            data.into_iter().map(|(r, c)| Position { r, c }).collect();

        let mut direction = NORTH;

        let mut round = 0;
        loop {
            round += 1;
            let mut new_positions = HashSet::new();

            let mut movement_propositions = HashMap::new();
            for pos in positions.iter().cloned() {
                let do_move = get_neighbours(pos)
                    .into_iter()
                    .any(|p| positions.contains(&p));

                if do_move {
                    let mut moved = false;
                    for n in 0..4 {
                        let dir = (direction + n) % 4;
                        let dir_neighbours = match dir {
                            NORTH => get_neighbours_north(pos),
                            SOUTH => get_neighbours_south(pos),
                            WEST => get_neighbours_west(pos),
                            EAST => get_neighbours_east(pos),
                            _ => unreachable!(),
                        };

                        let can_move =
                            !(dir_neighbours.into_iter().any(|p| positions.contains(&p)));

                        if can_move {
                            movement_propositions.insert(pos, mov(pos, dir));
                            moved = true;
                            break;
                        }
                    }
                    // not sure if necessary, rules don't seem to mention it
                    if !moved {
                        new_positions.insert(pos);
                    }
                } else {
                    new_positions.insert(pos);
                }
            }

            let mut grouped_propositions: HashMap<Position, usize> = HashMap::new();
            for (_, t) in &movement_propositions {
                grouped_propositions
                    .entry(*t)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }

            let mut any_moved = false;
            for (f, t) in movement_propositions {
                if grouped_propositions[&t] == 1 {
                    new_positions.insert(t);
                    any_moved = true;
                } else {
                    new_positions.insert(f);
                }
            }

            if !any_moved {
                break;
            }

            positions = new_positions;
            direction += 1;
            direction %= 4;
        }

        round
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 20);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 935);
        Ok(())
    }
}
