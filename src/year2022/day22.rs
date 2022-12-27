#[cfg(test)]
mod day22 {
    use std::collections::HashMap;

    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day22::{get_data, Move},
    };

    const RIGHT: usize = 0;
    const DOWN: usize = 1;
    const LEFT: usize = 2;
    const UP: usize = 3;

    fn a(data: (Vec<Vec<char>>, Vec<Move>)) -> usize {
        let (map, moves) = data;
        let map_h = map.len() as i32;
        let map_w = map[0].len() as i32;
        let mut orientation = 0;
        let mut position = (
            0,
            map[0]
                .iter()
                .enumerate()
                .find(|(_, c)| **c == '.')
                .map(|(i, _)| i)
                .unwrap(),
        );

        for mov in moves {
            match mov {
                Move::TurnRight => {
                    orientation += 1;
                    orientation %= 4;
                }
                Move::TurnLeft => {
                    // 3 == -1 mod 4
                    orientation += 3;
                    orientation %= 4;
                }
                Move::Walk(n) => {
                    let dir: (i32, i32) = match orientation {
                        RIGHT => (0, 1),
                        DOWN => (1, 0),
                        LEFT => (0, -1),
                        UP => (-1, 0),
                        _ => unreachable!(),
                    };

                    let mut steps_left = n;
                    let mut phantom_position = position;
                    while steps_left > 0 {
                        let new_position = (
                            phantom_position.0 as i32 + dir.0,
                            phantom_position.1 as i32 + dir.1,
                        );
                        let new_position = (new_position.0 + map_h, new_position.1 + map_w);
                        let new_position = (new_position.0 % map_h, new_position.1 % map_w);
                        let new_position = (new_position.0 as usize, new_position.1 as usize);

                        let tile = map[new_position.0][new_position.1];

                        match tile {
                            '#' => break,
                            '.' => {
                                position = new_position;
                                phantom_position = new_position;
                                steps_left -= 1;
                            }
                            ' ' => {
                                phantom_position = new_position;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        1000 * (position.0 + 1) + 4 * (position.1 + 1) + orientation
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 6032);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 30552);
        Ok(())
    }

    fn b(
        data: (Vec<Vec<char>>, Vec<Move>),
        side_correlation_map: HashMap<((usize, usize), usize), ((usize, usize), usize)>,
    ) -> usize {
        let (map, moves) = data;
        let map_h = map.len() as i32;
        let map_w = map[0].len() as i32;
        let mut orientation = 0;
        let mut position = (
            0,
            map[0]
                .iter()
                .enumerate()
                .find(|(_, c)| **c == '.')
                .map(|(i, _)| i)
                .unwrap(),
        );

        for mov in moves {
            match mov {
                Move::TurnRight => {
                    orientation += 1;
                    orientation %= 4;
                }
                Move::TurnLeft => {
                    // 3 == -1 mod 4
                    orientation += 3;
                    orientation %= 4;
                }
                Move::Walk(n) => {
                    let dir = |orientation| match orientation {
                        RIGHT => (0, 1),
                        DOWN => (1, 0),
                        LEFT => (0, -1),
                        UP => (-1, 0),
                        _ => unreachable!(),
                    };

                    for _ in 0..n {
                        let new_position;
                        let new_orientation;
                        if side_correlation_map.contains_key(&(position, orientation)) {
                            new_position = side_correlation_map[&(position, orientation)].0;
                            new_orientation = side_correlation_map[&(position, orientation)].1;
                        } else {
                            let d = dir(orientation);
                            let pos = (position.0 as i32 + d.0, position.1 as i32 + d.1);
                            let pos = (pos.0 + map_h, pos.1 + map_w);
                            let pos = (pos.0 % map_h, pos.1 % map_w);
                            new_position = (pos.0 as usize, pos.1 as usize);
                            new_orientation = orientation;
                        }

                        let tile = map[new_position.0][new_position.1];

                        if tile == '#' {
                            break;
                        } else {
                            position = new_position;
                            orientation = new_orientation;
                        }
                    }
                }
            }
        }

        1000 * (position.0 + 1) + 4 * (position.1 + 1) + orientation
    }

    // TODO: figure out a way to find the side gluings
    // idea #1: use known situations like small and big L to find some gluings
    // and use trace paths to find out the mapping and orientation of the rest
    // idea #2: find/analyse all possible shapes of cube "templates"
    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        let dir_down: (i32, i32) = (1, 0);
        let dir_up: (i32, i32) = (-1, 0);
        let dir_right: (i32, i32) = (0, 1);
        let dir_left: (i32, i32) = (0, -1);

        let mut side_correlation_map = HashMap::new();
        let side_len = 4;
        let mappings = vec![
            ((3, 8), dir_up, (LEFT, DOWN), (4, 7), dir_left, (UP, RIGHT)), //A
            (
                (7, 11),
                dir_up,
                (RIGHT, DOWN),
                (8, 12),
                dir_right,
                (UP, LEFT),
            ), //B
            (
                (7, 7),
                dir_left,
                (DOWN, RIGHT),
                (8, 8),
                dir_down,
                (LEFT, UP),
            ), // C
            (
                (3, 11),
                dir_up,
                (RIGHT, LEFT),
                (8, 15),
                dir_down,
                (RIGHT, LEFT),
            ), // D
            ((0, 8), dir_right, (UP, DOWN), (4, 3), dir_left, (UP, DOWN)), // E
            ((7, 3), dir_left, (DOWN, UP), (11, 8), dir_right, (DOWN, UP)), //F
            (
                (4, 0),
                dir_down,
                (LEFT, UP),
                (11, 15),
                dir_left,
                (DOWN, RIGHT),
            ), // G
        ];

        for (p_a, d_a, o_a, p_b, d_b, o_b) in mappings {
            let mut pos_a = p_a;
            let mut pos_b = p_b;

            for _ in 0..side_len {
                side_correlation_map.insert(
                    ((pos_a.0 as usize, pos_a.1 as usize), o_a.0),
                    ((pos_b.0 as usize, pos_b.1 as usize), o_a.1),
                );
                side_correlation_map.insert(
                    ((pos_b.0 as usize, pos_b.1 as usize), o_b.0),
                    ((pos_a.0 as usize, pos_a.1 as usize), o_b.1),
                );

                pos_a = (pos_a.0 + d_a.0, pos_a.1 + d_a.1);
                pos_b = (pos_b.0 + d_b.0, pos_b.1 + d_b.1);
            }
        }

        assert_eq!(b(data, side_correlation_map), 5031);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        let dir_down: (i32, i32) = (1, 0);
        let dir_up: (i32, i32) = (-1, 0);
        let dir_right: (i32, i32) = (0, 1);
        let dir_left: (i32, i32) = (0, -1);

        let mut side_correlation_map = HashMap::new();
        let side_len = 50;
        let mappings = vec![
            (
                (99, 50),
                dir_up,
                (LEFT, DOWN),
                (100, 49),
                dir_left,
                (UP, RIGHT),
            ), //A
            (
                (50, 99),
                dir_down,
                (RIGHT, UP),
                (49, 100),
                dir_right,
                (DOWN, LEFT),
            ), //B
            (
                (150, 49),
                dir_down,
                (RIGHT, UP),
                (149, 50),
                dir_right,
                (DOWN, LEFT),
            ), //C
            (
                (100, 99),
                dir_down,
                (RIGHT, LEFT),
                (49, 149),
                dir_up,
                (RIGHT, LEFT),
            ), //D
            (
                (100, 0),
                dir_down,
                (LEFT, RIGHT),
                (49, 50),
                dir_up,
                (LEFT, RIGHT),
            ), //E
            (
                (150, 0),
                dir_down,
                (LEFT, DOWN),
                (0, 50),
                dir_right,
                (UP, RIGHT),
            ), //F
            (
                (199, 0),
                dir_right,
                (DOWN, DOWN),
                (0, 100),
                dir_right,
                (UP, UP),
            ), //G
        ];

        for (p_a, d_a, o_a, p_b, d_b, o_b) in mappings {
            let mut pos_a = p_a;
            let mut pos_b = p_b;

            for _ in 0..side_len {
                side_correlation_map.insert(
                    ((pos_a.0 as usize, pos_a.1 as usize), o_a.0),
                    ((pos_b.0 as usize, pos_b.1 as usize), o_a.1),
                );
                side_correlation_map.insert(
                    ((pos_b.0 as usize, pos_b.1 as usize), o_b.0),
                    ((pos_a.0 as usize, pos_a.1 as usize), o_b.1),
                );

                pos_a = (pos_a.0 + d_a.0, pos_a.1 + d_a.1);
                pos_b = (pos_b.0 + d_b.0, pos_b.1 + d_b.1);
            }
        }

        assert_eq!(b(data, side_correlation_map), 184106);
        Ok(())
    }
}
